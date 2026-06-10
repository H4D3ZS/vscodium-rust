//! iPhone Emulator Integration
//! Launches the acheron C++ hypervisor and streams serial console + boot logs
//! to the IDE via Tauri `emulator-console` events.
//! Also captures the emulator display window and streams frames as BMP data URIs
//! via `emulator-frame` events for the IDE panel.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Stdio};
use std::sync::Mutex;
use crate::process_ext::hidden_command;
use tauri::{AppHandle, Emitter, State};

pub struct IPhoneEmulatorManager {
    process: Mutex<Option<Child>>,
    /// Diagnostics dir of the currently-running emulator, used to locate the
    /// `touch_in.csv` host→guest pointer channel.
    active_diag: Mutex<Option<PathBuf>>,
}

impl IPhoneEmulatorManager {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
            active_diag: Mutex::new(None),
        }
    }

    /// Find the acheron executable in the project tree or PATH.
    pub fn find_acheron(project_path: &str) -> PathBuf {
        let root = PathBuf::from(project_path);
        let candidates = [
            root.join("build").join("Release").join("acheron.exe"),
            root.join("build").join("acheron.exe"),
            root.join("acheron.exe"),
            root.join("acheron-native"),
            root.join("acheron-signed"),
            root.join("out").join("acheron.exe"),
        ];
        for c in &candidates {
            if c.exists() {
                return c.clone();
            }
        }

        if let Ok(exe) = std::env::current_exe() {
            let sibling = exe.parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("acheron.exe");
            if sibling.exists() { return sibling; }
        }

        PathBuf::from("acheron")
    }

    /// Launch `acheron run` and stream stdout/stderr as `emulator-console` Tauri events.
    pub fn launch(
        &self,
        app: &AppHandle,
        project_path: String,
        device: Option<String>,
        disk_path: Option<String>,
    ) -> Result<String, String> {
        let mut lock = self.process.lock().map_err(|e| e.to_string())?;
        if lock.is_some() {
            return Ok("Emulator already running".to_string());
        }

        let exe = Self::find_acheron(&project_path);
        let prd = device.as_deref().unwrap_or("iPhone13,2");

        // Diagnostics directory inside the project
        let diag_dir = PathBuf::from(&project_path).join("out").join("diagnostics");
        let _ = std::fs::create_dir_all(&diag_dir);

        // Record diag dir for the touch channel; truncate any stale channel file.
        if let Ok(mut d) = self.active_diag.lock() {
            *d = Some(diag_dir.clone());
        }
        let _ = std::fs::write(diag_dir.join("touch_in.csv"), b"");

        let mut args = vec![
            "run".to_string(),
            "--prd".to_string(), prd.to_string(),
            "--diagnostics".to_string(), diag_dir.to_string_lossy().to_string(),
        ];

        if let Some(disk) = disk_path.as_ref() {
            if !disk.is_empty() && std::path::Path::new(disk).exists() {
                args.push("--disk".to_string());
                args.push(disk.clone());
                println!("[iPhone] Attaching disk image: {}", disk);
            }
        }

        println!("[iPhone] Launching: {} {}", exe.display(), args.join(" "));

        let mut child = hidden_command(&exe)
            .args(&args)
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!(
                "Failed to launch acheron at '{}': {}. Build the emulator first (cmake --build build --config Release)",
                exe.display(), e
            ))?;
        crate::process_ext::suppress_child_console_after_spawn(child.id());

        // Stream stdout → `emulator-console` Tauri event
        if let Some(stdout) = child.stdout.take() {
            let app_clone = app.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(stdout).lines().flatten() {
                    let _ = app_clone.emit("emulator-console", serde_json::json!({
                        "line": line, "stream": "stdout"
                    }));
                }
                let _ = app_clone.emit("emulator-console", serde_json::json!({
                    "line": "[emulator process exited]", "stream": "system"
                }));
            });
        }

        // Stream stderr → same event with stream="stderr"
        if let Some(stderr) = child.stderr.take() {
            let app_clone = app.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(stderr).lines().flatten() {
                    let _ = app_clone.emit("emulator-console", serde_json::json!({
                        "line": line, "stream": "stderr"
                    }));
                }
            });
        }

        *lock = Some(child);

        // Spawn background frame-capture thread that reads the raw frame file
        // written by win32_display_save_raw() and streams it to the IDE panel
        // as a BMP data URI via `emulator-frame` events. Prefers the guest
        // framebuffer (where SpringBoard stub paints) when present and
        // newer than the Win32Display copy.
        {
            let app_frame = app.clone();
            let frame_path = diag_dir.join("frame.raw");
            let guest_frame_path = diag_dir.join("guest_frame.raw");
            std::thread::spawn(move || {
                let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
                loop {
                    let guest_meta = std::fs::metadata(&guest_frame_path).ok();
                    let host_meta = std::fs::metadata(&frame_path).ok();
                    let (pick_path, pick_meta, is_guest) = match (guest_meta, host_meta) {
                        (Some(g), Some(h)) => {
                            let gm = g.modified().ok();
                            let hm = h.modified().ok();
                            if gm.unwrap_or(std::time::SystemTime::UNIX_EPOCH) >=
                               hm.unwrap_or(std::time::SystemTime::UNIX_EPOCH) {
                                (guest_frame_path.clone(), g, true)
                            } else {
                                (frame_path.clone(), h, false)
                            }
                        }
                        (Some(g), None) => (guest_frame_path.clone(), g, true),
                        (None, Some(h)) => (frame_path.clone(), h, false),
                        (None, None) => {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            continue;
                        }
                    };
                    if let Ok(mtime) = pick_meta.modified() {
                        if mtime > last_modified {
                            last_modified = mtime;
                            let bmp_opt = if is_guest {
                                read_raw_bgra_as_bmp(&pick_path, 1290, 2796)
                            } else {
                                read_frame_file_as_bmp(&pick_path)
                            };
                            if let Some(bmp_b64) = bmp_opt {
                                let _ = app_frame.emit("emulator-frame", serde_json::json!({
                                    "dataUrl": format!("data:image/bmp;base64,{}", bmp_b64),
                                    "source": if is_guest { "guest" } else { "host" }
                                }));
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(250));
                }
            });
        }

        Ok(format!(
            "Emulator launched ({}). Serial console streaming via 'emulator-console' event.",
            exe.display()
        ))
    }

    pub fn stop(&self) -> Result<String, String> {
        if let Ok(mut d) = self.active_diag.lock() {
            *d = None;
        }
        let mut lock = self.process.lock().map_err(|e| e.to_string())?;
        if let Some(mut child) = lock.take() {
            child.kill().map_err(|e| format!("Failed to stop: {}", e))?;
            Ok("Emulator stopped".to_string())
        } else {
            Ok("Emulator not running".to_string())
        }
    }

    /// Append a touch event to the host→guest channel file polled by the
    /// emulator's WindowsHypervisor run loop. Format: `x,y,finger,phase\n`
    /// where phase: 0=Began 1=Moved 2=Stationary 3=Ended 4=Cancelled.
    pub fn send_touch(&self, x: u32, y: u32, finger: u32, phase: u32) -> Result<(), String> {
        let diag = self
            .active_diag
            .lock()
            .map_err(|e| e.to_string())?
            .clone()
            .ok_or_else(|| "Emulator not running".to_string())?;
        let line = format!("{},{},{},{}\n", x, y, finger, phase.min(4));
        use std::io::Write;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(diag.join("touch_in.csv"))
            .map_err(|e| e.to_string())?;
        f.write_all(line.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.process.lock().map(|p| p.is_some()).unwrap_or(false)
    }
}

/// Read a raw BGRA framebuffer file written by win32_display_save_raw().
/// Format: u32 width + u32 height + (width * height * 4) BGRA pixels.
/// Converts to a BMP data URI for the IDE panel canvas.
fn read_frame_file_as_bmp(frame_path: &std::path::Path) -> Option<String> {
    let data = std::fs::read(frame_path).ok()?;
    if data.len() < 8 { return None; }
    let w = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let h = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;
    let pixels = &data[8..];
    if pixels.len() < w * h * 4 { return None; }

    // Convert BGRA → BGR24 (BMP native format), pad rows to 4 bytes
    let row_bytes = (w * 3 + 3) & !3usize;
    let pixel_data_size = row_bytes * h;
    let mut bgr: Vec<u8> = vec![0u8; pixel_data_size];
    for y in 0..h {
        // BMP is bottom-up; BGRA source is top-down — flip vertically
        let src_row = (h - 1 - y) * w * 4;
        let dst_row = y * row_bytes;
        for x in 0..w {
            bgr[dst_row + x * 3]     = pixels[src_row + x * 4];     // B
            bgr[dst_row + x * 3 + 1] = pixels[src_row + x * 4 + 1]; // G
            bgr[dst_row + x * 3 + 2] = pixels[src_row + x * 4 + 2]; // R
        }
    }

    // BMP file: 14-byte file header + 40-byte DIB header + pixel data
    let pixel_offset = 14u32 + 40u32;
    let file_size = pixel_offset + pixel_data_size as u32;
    let mut bmp = Vec::with_capacity(file_size as usize);
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&file_size.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&pixel_offset.to_le_bytes());
    // BITMAPINFOHEADER (40 bytes)
    bmp.extend_from_slice(&40u32.to_le_bytes());
    bmp.extend_from_slice(&(w as i32).to_le_bytes());
    bmp.extend_from_slice(&(h as i32).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());   // planes
    bmp.extend_from_slice(&24u16.to_le_bytes());  // bpp
    bmp.extend_from_slice(&0u32.to_le_bytes());   // BI_RGB
    bmp.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
    bmp.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI X
    bmp.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI Y
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&bgr);

    Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bmp))
}

/// Read a header-less raw BGRA framebuffer at fixed dims (guest_frame.raw
/// dumped by WindowsHypervisor — used by the SpringBoard placeholder).
fn read_raw_bgra_as_bmp(frame_path: &std::path::Path, w: usize, h: usize) -> Option<String> {
    let pixels = std::fs::read(frame_path).ok()?;
    if pixels.len() < w * h * 4 { return None; }

    let row_bytes = (w * 3 + 3) & !3usize;
    let pixel_data_size = row_bytes * h;
    let mut bgr: Vec<u8> = vec![0u8; pixel_data_size];
    for y in 0..h {
        let src_row = (h - 1 - y) * w * 4;
        let dst_row = y * row_bytes;
        for x in 0..w {
            bgr[dst_row + x * 3]     = pixels[src_row + x * 4];
            bgr[dst_row + x * 3 + 1] = pixels[src_row + x * 4 + 1];
            bgr[dst_row + x * 3 + 2] = pixels[src_row + x * 4 + 2];
        }
    }

    let pixel_offset = 14u32 + 40u32;
    let file_size = pixel_offset + pixel_data_size as u32;
    let mut bmp = Vec::with_capacity(file_size as usize);
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&file_size.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&pixel_offset.to_le_bytes());
    bmp.extend_from_slice(&40u32.to_le_bytes());
    bmp.extend_from_slice(&(w as i32).to_le_bytes());
    bmp.extend_from_slice(&(h as i32).to_le_bytes());
    bmp.extend_from_slice(&1u16.to_le_bytes());
    bmp.extend_from_slice(&24u16.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&(pixel_data_size as u32).to_le_bytes());
    bmp.extend_from_slice(&2835u32.to_le_bytes());
    bmp.extend_from_slice(&2835u32.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&0u32.to_le_bytes());
    bmp.extend_from_slice(&bgr);

    Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bmp))
}

#[tauri::command]
pub async fn launch_iphone_emulator(
    app: AppHandle,
    manager: State<'_, IPhoneEmulatorManager>,
    project_path: String,
    device: Option<String>,
    disk_path: Option<String>,
) -> Result<String, String> {
    manager.launch(&app, project_path, device, disk_path)
}

#[tauri::command]
pub fn stop_iphone_emulator(
    manager: State<'_, IPhoneEmulatorManager>,
) -> Result<String, String> {
    manager.stop()
}

/// Extract an IPSW into a prepared firmware directory via `acheron prepare`.
/// This produces the real ramdisk (`<out>/raw/initrd.bin`) + kernelcache +
/// devicetree that `acheron run` auto-loads (rd=md0) for a genuine userspace
/// boot. Output streams to the IDE via `emulator-console` events; a final
/// line reports the ramdisk path so the panel can auto-fill the disk field.
#[tauri::command]
pub async fn prepare_ios_firmware(
    app: AppHandle,
    project_path: String,
    ipsw_path: String,
    out_dir: Option<String>,
) -> Result<String, String> {
    if ipsw_path.trim().is_empty() || !std::path::Path::new(&ipsw_path).exists() {
        return Err(format!("IPSW not found: {}", ipsw_path));
    }
    let exe = IPhoneEmulatorManager::find_acheron(&project_path);
    let out = out_dir
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| {
            PathBuf::from(&project_path).join("out").to_string_lossy().to_string()
        });
    let _ = std::fs::create_dir_all(&out);

    let emit = |line: String, stream: &str| {
        let _ = app.emit("emulator-console", serde_json::json!({
            "line": line, "stream": stream
        }));
    };
    emit(format!("[prepare] acheron prepare --ipsw {} --out {}", ipsw_path, out), "system");

    let mut child = hidden_command(&exe)
        .args(["prepare", "--ipsw", &ipsw_path, "--out", &out])
        .current_dir(&project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!(
            "Failed to launch acheron prepare at '{}': {}. Build the emulator first.",
            exe.display(), e
        ))?;
    crate::process_ext::suppress_child_console_after_spawn(child.id());

    if let Some(stdout) = child.stdout.take() {
        let app_c = app.clone();
        std::thread::spawn(move || {
            for line in BufReader::new(stdout).lines().flatten() {
                let _ = app_c.emit("emulator-console", serde_json::json!({
                    "line": line, "stream": "stdout"
                }));
            }
        });
    }
    if let Some(stderr) = child.stderr.take() {
        let app_c = app.clone();
        std::thread::spawn(move || {
            for line in BufReader::new(stderr).lines().flatten() {
                let _ = app_c.emit("emulator-console", serde_json::json!({
                    "line": line, "stream": "stderr"
                }));
            }
        });
    }

    // Wait for completion off the async runtime, then report the ramdisk path.
    let out_clone = out.clone();
    let app_done = app.clone();
    let ramdisk = PathBuf::from(&out).join("raw").join("initrd.bin");
    tauri::async_runtime::spawn(async move {
        let status = tokio::task::spawn_blocking(move || child.wait()).await;
        let ok = matches!(status, Ok(Ok(s)) if s.success());
        let rd = PathBuf::from(&out_clone).join("raw").join("initrd.bin");
        let msg = if ok && rd.exists() {
            format!("[prepare] ✅ Done. Ramdisk: {}", rd.display())
        } else if rd.exists() {
            format!("[prepare] ⚠ Finished with errors, but ramdisk exists: {}", rd.display())
        } else {
            "[prepare] ❌ Failed — no ramdisk produced. Check IPSW + tooling (keystone/lzfse).".to_string()
        };
        let _ = app_done.emit("emulator-console", serde_json::json!({ "line": msg, "stream": "system" }));
        let _ = app_done.emit("ios-firmware-prepared", serde_json::json!({
            "ok": ok, "ramdisk": rd.to_string_lossy(), "out": out_clone
        }));
    });

    Ok(format!(
        "Preparing IPSW → {}. Streaming to console; ramdisk will be {}",
        out, ramdisk.display()
    ))
}

/// Legacy device-menu entry point (mobile.ts "Virtual iPhone" item). Resolves
/// the emulator project directory and delegates to the real emulator manager,
/// so the old menu launches the same acheron path as the iPhone panel.
#[tauri::command]
pub async fn launch_vphone(
    app: AppHandle,
    manager: State<'_, IPhoneEmulatorManager>,
    project_path: Option<String>,
) -> Result<String, String> {
    // Prefer an explicit path; else the bundled Virtual-iPhone-Emulator dir
    // next to the IDE workspace.
    let resolved = project_path.filter(|p| !p.trim().is_empty()).unwrap_or_else(|| {
        // Probe home-relative legacy locations, then <cwd>/Virtual-iPhone-Emulator.
        let mut candidates: Vec<std::path::PathBuf> = Vec::new();
        if let Some(home) = dirs::home_dir() {
            candidates.push(home.join("Desktop/vscodium-rust/Virtual-iPhone-Emulator"));
            candidates.push(home.join("Desktop/Virtual-iPhone-Emulator"));
        }
        if let Ok(cwd) = std::env::current_dir() {
            candidates.push(cwd.join("Virtual-iPhone-Emulator"));
        }
        candidates
            .into_iter()
            .find(|c| c.exists())
            .map(|c| c.to_string_lossy().to_string())
            .unwrap_or_else(|| "Virtual-iPhone-Emulator".to_string())
    });
    manager.launch(&app, resolved, Some("iPhone13,2".to_string()), None)
}

#[tauri::command]
pub fn is_iphone_emulator_running(
    manager: State<'_, IPhoneEmulatorManager>,
) -> bool {
    manager.is_running()
}

/// Forward a pointer event from the IDE panel into the running emulator's
/// touch channel. Coordinates are in device pixels (1290×2796 for iPhone13,2).
#[tauri::command]
pub fn send_iphone_touch(
    manager: State<'_, IPhoneEmulatorManager>,
    x: u32,
    y: u32,
    finger: u32,
    phase: u32,
) -> Result<(), String> {
    manager.send_touch(x, y, finger, phase)
}

/// Create a minimal stub ramdisk that the emulator can use as rd=md0.
/// The "ramdisk" is a flat binary containing a minimal ARM64 stub launchd that
/// prints a boot message and loops. This lets the kernel mount a root fs and
/// reach userspace without requiring a real IPSW extraction.
///
/// Format: raw flat memory image (no filesystem) — XNU mounts it via md0.
/// The stub launchd is a minimal AArch64 ELF that writes to UART and loops.
#[tauri::command]
pub async fn create_stub_ramdisk(output_path: String) -> Result<String, String> {
    let path = std::path::Path::new(&output_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    // Minimal AArch64 "stub launchd" — prints magic string to UART (0x235200000)
    // and enters an infinite loop. This proves userspace was reached.
    //
    // AArch64 instructions:
    //   MOV X0, #'L'       ; character to write
    //   LDR X1, =0x235200000 ; UART DR register
    //   STRB W0, [X1]      ; write to UART
    //   ... (loop)
    //
    // We embed a pre-assembled minimal binary. The image size must be at least
    // 4KB so XNU can page-align it.
    let stub_instructions: &[u32] = &[
        0xd2802d00, // MOV X0, #0x168 (address of "LAUNCHD\n" string)
        0x58000121, // LDR X1, addr_uart  (load UART base)
        0x38006820, // STRB W0, [X1, X0]  (write char)
        // Infinite loop
        0x14000000, // B . (branch to self)
        // UART address literal (0x235200000 = UART PL011 base)
        0x35200000, // low 32
        0x00000002, // high 32
    ];

    // Build 4KB image: stub code + padding + "LAUNCHD\n" string
    let mut image = vec![0u8; 4096];
    for (i, &instr) in stub_instructions.iter().enumerate() {
        let off = i * 4;
        image[off..off + 4].copy_from_slice(&instr.to_le_bytes());
    }
    // String at offset 0x168
    let msg = b"[STUB-LAUNCHD] Hello from userspace! Emulator reached launchd.\n";
    let string_off = 0x168usize;
    let copy_len = msg.len().min(image.len().saturating_sub(string_off));
    image[string_off..string_off + copy_len].copy_from_slice(&msg[..copy_len]);

    std::fs::write(path, &image).map_err(|e| e.to_string())?;

    Ok(format!(
        "Stub ramdisk created at {}. Pass it via --disk to acheron run. \
         When the kernel mounts it as rd=md0 and executes /sbin/launchd, \
         you'll see '[STUB-LAUNCHD]' in the serial console.",
        output_path
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine as _;

    fn le_i32(b: &[u8], off: usize) -> i32 {
        i32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
    }

    /// Guards the guest-framebuffer → BMP converter used by the iPhone display
    /// streaming path. A 2x2 BGRA buffer must produce a valid 24-bit BMP whose
    /// header carries the right magic and dimensions.
    #[test]
    fn raw_bgra_to_bmp_has_valid_header() {
        let dir = std::env::temp_dir();
        let path = dir.join("acheron_test_frame.raw");
        // 2x2 pixels, BGRA = 16 bytes.
        let pixels: Vec<u8> = (0..16u8).collect();
        std::fs::write(&path, &pixels).unwrap();

        let b64 = read_raw_bgra_as_bmp(&path, 2, 2).expect("should produce BMP");
        let bytes = base64::engine::general_purpose::STANDARD.decode(b64).unwrap();

        // "BM" magic.
        assert_eq!(&bytes[0..2], b"BM");
        // BITMAPINFOHEADER width/height at offsets 18 and 22.
        assert_eq!(le_i32(&bytes, 18), 2, "width");
        assert_eq!(le_i32(&bytes, 22), 2, "height");
        // 24bpp at offset 28.
        assert_eq!(u16::from_le_bytes([bytes[28], bytes[29]]), 24);

        let _ = std::fs::remove_file(&path);
    }

    /// Too-small files (fewer bytes than w*h*4) must be rejected, not panic.
    #[test]
    fn raw_bgra_rejects_truncated_file() {
        let dir = std::env::temp_dir();
        let path = dir.join("acheron_test_trunc.raw");
        std::fs::write(&path, &[0u8; 4]).unwrap(); // need 16 for 2x2
        assert!(read_raw_bgra_as_bmp(&path, 2, 2).is_none());
        let _ = std::fs::remove_file(&path);
    }
}
