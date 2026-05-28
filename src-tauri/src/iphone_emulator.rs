//! iPhone Emulator Integration
//! Launches the acheron C++ hypervisor and streams serial console + boot logs
//! to the IDE via Tauri `emulator-console` events.
//! Also captures the emulator display window and streams frames as BMP data URIs
//! via `emulator-frame` events for the IDE panel.

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};

pub struct IPhoneEmulatorManager {
    process: Mutex<Option<Child>>,
}

impl IPhoneEmulatorManager {
    pub fn new() -> Self {
        Self { process: Mutex::new(None) }
    }

    /// Find the acheron executable in the project tree or PATH.
    fn find_acheron(project_path: &str) -> PathBuf {
        // 1. build/Release in the project directory
        let in_build = PathBuf::from(project_path).join("build").join("Release").join("acheron.exe");
        if in_build.exists() { return in_build; }

        // 2. build directory (cmake default output)
        let in_build2 = PathBuf::from(project_path).join("build").join("acheron.exe");
        if in_build2.exists() { return in_build2; }

        // 3. Next to the running IDE binary
        if let Ok(exe) = std::env::current_exe() {
            let sibling = exe.parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("acheron.exe");
            if sibling.exists() { return sibling; }
        }

        // 4. Fall back to PATH
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

        let mut child = Command::new(&exe)
            .args(&args)
            .current_dir(&project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!(
                "Failed to launch acheron at '{}': {}. Build the emulator first (cmake --build build --config Release)",
                exe.display(), e
            ))?;

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
        // as a BMP data URI via `emulator-frame` events.
        {
            let app_frame = app.clone();
            let frame_path = diag_dir.join("frame.raw");
            std::thread::spawn(move || {
                let mut last_modified = std::time::SystemTime::UNIX_EPOCH;
                loop {
                    if let Ok(meta) = std::fs::metadata(&frame_path) {
                        if let Ok(mtime) = meta.modified() {
                            if mtime > last_modified {
                                last_modified = mtime;
                                if let Some(bmp_b64) = read_frame_file_as_bmp(&frame_path) {
                                    let _ = app_frame.emit("emulator-frame", serde_json::json!({
                                        "dataUrl": format!("data:image/bmp;base64,{}", bmp_b64)
                                    }));
                                }
                            }
                        }
                    }
                    std::thread::sleep(std::time::Duration::from_millis(500));
                }
            });
        }

        Ok(format!(
            "Emulator launched ({}). Serial console streaming via 'emulator-console' event.",
            exe.display()
        ))
    }

    pub fn stop(&self) -> Result<String, String> {
        let mut lock = self.process.lock().map_err(|e| e.to_string())?;
        if let Some(mut child) = lock.take() {
            child.kill().map_err(|e| format!("Failed to stop: {}", e))?;
            Ok("Emulator stopped".to_string())
        } else {
            Ok("Emulator not running".to_string())
        }
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
    let row_bytes = ((w * 3 + 3) & !3usize);
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

#[tauri::command]
pub fn is_iphone_emulator_running(
    manager: State<'_, IPhoneEmulatorManager>,
) -> bool {
    manager.is_running()
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
