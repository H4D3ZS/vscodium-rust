use serde_json::{json, Value};

pub fn capture_main_screenshot(_app: &tauri::AppHandle) -> Result<Value, String> {
    capture_screen_impl()
}

/// Capture the primary monitor and return a BMP data URI + dimensions.
/// Real implementation (Win32 GDI on Windows); other platforms report
/// unsupported rather than returning fake data.
#[tauri::command]
pub async fn capture_preview_screenshot() -> Result<Value, String> {
    capture_screen_impl()
}

#[cfg(target_os = "windows")]
fn capture_screen_impl() -> Result<Value, String> {
    use base64::{engine::general_purpose, Engine as _};
    use windows::Win32::Foundation::HWND;
    use windows::Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
        HBITMAP, HGDIOBJ, SRCCOPY,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

    unsafe {
        let w = GetSystemMetrics(SM_CXSCREEN);
        let h = GetSystemMetrics(SM_CYSCREEN);
        if w <= 0 || h <= 0 {
            return Err("Failed to read screen metrics".into());
        }

        let screen_dc = GetDC(HWND(std::ptr::null_mut()));
        if screen_dc.0.is_null() {
            return Err("GetDC(screen) failed".into());
        }
        let mem_dc = CreateCompatibleDC(screen_dc);
        let bitmap: HBITMAP = CreateCompatibleBitmap(screen_dc, w, h);
        let old = SelectObject(mem_dc, HGDIOBJ(bitmap.0));

        let blt = BitBlt(mem_dc, 0, 0, w, h, screen_dc, 0, 0, SRCCOPY);

        // Top-down 32-bit BGRA via negative height.
        let mut bi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h, // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        let scanned = GetDIBits(
            mem_dc,
            bitmap,
            0,
            h as u32,
            Some(pixels.as_mut_ptr() as *mut _),
            &mut bi,
            DIB_RGB_COLORS,
        );

        // Cleanup GDI objects.
        SelectObject(mem_dc, old);
        let _ = DeleteObject(HGDIOBJ(bitmap.0));
        let _ = DeleteDC(mem_dc);
        ReleaseDC(HWND(std::ptr::null_mut()), screen_dc);

        if blt.is_err() || scanned == 0 {
            return Err("Screen BitBlt/GetDIBits failed".into());
        }

        let bmp = encode_bgra_bmp(&pixels, w as u32, h as u32);
        let b64 = general_purpose::STANDARD.encode(&bmp);
        Ok(json!({
            "ok": true,
            "width": w,
            "height": h,
            "dataUrl": format!("data:image/bmp;base64,{}", b64),
        }))
    }
}

#[cfg(not(target_os = "windows"))]
fn capture_screen_impl() -> Result<Value, String> {
    Ok(json!({
        "ok": false,
        "message": "Desktop screen capture is implemented for Windows; this platform is not yet supported."
    }))
}

/// Encode a top-down BGRA buffer as a 24-bit BMP (bottom-up, row-padded).
#[cfg(target_os = "windows")]
fn encode_bgra_bmp(pixels: &[u8], w: u32, h: u32) -> Vec<u8> {
    let (wu, hu) = (w as usize, h as usize);
    let row_bytes = (wu * 3 + 3) & !3usize;
    let pixel_data_size = row_bytes * hu;
    let mut bgr = vec![0u8; pixel_data_size];
    for y in 0..hu {
        // BMP is bottom-up; source is top-down → flip vertically.
        let src_row = (hu - 1 - y) * wu * 4;
        let dst_row = y * row_bytes;
        for x in 0..wu {
            bgr[dst_row + x * 3] = pixels[src_row + x * 4];     // B
            bgr[dst_row + x * 3 + 1] = pixels[src_row + x * 4 + 1]; // G
            bgr[dst_row + x * 3 + 2] = pixels[src_row + x * 4 + 2]; // R
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
    bmp
}
