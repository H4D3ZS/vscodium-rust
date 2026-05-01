//! HADES Real-Time Vision System - Simplified Working Implementation

use std::sync::Arc;
use tokio::sync::RwLock;
use image::{DynamicImage, ImageBuffer, Rgba};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND, RECT},
    Graphics::Gdi::{BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteDC, DeleteObject, GetDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, SRCCOPY},
    UI::WindowsAndMessaging::{FindWindowA, GetWindowRect},
};

pub struct VisionState {
    pub last_analysis: String,
    pub last_update: u64,
    pub frames_captured: u32,
}

pub struct HadesVision {
    state: Arc<RwLock<VisionState>>,
    ollama_url: String,
    local_model: String,
    cloud_model: String,
    use_cloud: Arc<RwLock<bool>>,
}

impl HadesVision {
    pub fn new(ollama_url: &str, local_model: &str, cloud_model: &str, use_cloud: bool) -> Self {
        Self {
            state: Arc::new(RwLock::new(VisionState {
                last_analysis: String::new(),
                last_update: 0,
                frames_captured: 0,
            })),
            ollama_url: ollama_url.to_string(),
            local_model: local_model.to_string(),
            cloud_model: cloud_model.to_string(),
            use_cloud: Arc::new(RwLock::new(use_cloud)),
        }
    }

    #[cfg(target_os = "windows")]
    pub async fn capture_and_analyze(&self) -> Result<String, String> {
        use std::ffi::CString;
        use tokio::time::{Duration, Instant};
        use windows::core::PCSTR;
        
        let start = Instant::now();
        
        let emulator_titles = vec!["Android Emulator", "Pixel", "iPhone Simulator", "Virtual iPhone", "Acheron", "scrcpy"];
        let mut hwnd_opt = None;
        
        for title in emulator_titles {
            if let Ok(c_title) = CString::new(title) {
                unsafe {
                    let result = FindWindowA(PCSTR::null(), PCSTR::from_raw(c_title.as_ptr() as *const u8));
                    if let Ok(hwnd) = result {
                        hwnd_opt = Some(hwnd);
                        break;
                    }
                }
            }
        }

        let (x, y, w, h) = if let Some(hwnd) = hwnd_opt {
            let mut rect = RECT::default();
            unsafe { 
                let _ = GetWindowRect(hwnd, &mut rect);
            }
            (rect.left, rect.top, (rect.right - rect.left) as u32, (rect.bottom - rect.top) as u32)
        } else {
            (0, 0, 1024, 1024)
        };

        let frame_data = unsafe {
            let hdc_screen = GetDC(HWND(std::ptr::null_mut()));
            let hdc_mem = CreateCompatibleDC(hdc_screen);
            
            let mut bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: w as i32,
                    biHeight: -(h as i32),
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: 0,
                    biSizeImage: 0,
                    biXPelsPerMeter: 0,
                    biYPelsPerMeter: 0,
                    biClrUsed: 0,
                    biClrImportant: 0,
                },
                bmiColors: [Default::default()],
            };

            let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
            let hbitmap = CreateDIBSection(hdc_screen, &bmi, DIB_RGB_COLORS, &mut bits, None, 0);
            
            if let Ok(hbitmap) = hbitmap {
                let _ = SelectObject(hdc_mem, hbitmap);
                let _ = BitBlt(hdc_mem, 0, 0, w as i32, h as i32, hdc_screen, x, y, SRCCOPY);
                
                let size = (w * h * 4) as usize;
                let mut rgba = Vec::with_capacity(size);
                std::ptr::copy_nonoverlapping(bits, rgba.as_mut_ptr() as *mut _, size);
                rgba.set_len(size);
                
                let _ = DeleteObject(hbitmap);
                let _ = DeleteDC(hdc_mem);
                let _ = ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
                
                Some(rgba)
            } else {
                let _ = DeleteDC(hdc_mem);
                let _ = ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen);
                None
            }
        };

        if let Some(data) = frame_data {
            let target_size = if *self.use_cloud.read().await { 1024 } else { 512 };
            let resized = ImageBuffer::<Rgba<u8>, _>::from_raw(w, h, data.clone())
                .map(|img| {
                    let dyn_img = DynamicImage::ImageRgba8(img);
                    dyn_img.resize_exact(target_size, target_size, image::imageops::FilterType::Triangle).into_rgba8().into_raw()
                })
                .unwrap_or(data);

            // Use deprecated but working base64::encode
            #[allow(deprecated)]
            let base64_image = base64::encode(&resized);
            
            let model = if *self.use_cloud.read().await { self.cloud_model.clone() } else { self.local_model.clone() };

            let response = reqwest::Client::new()
                .post(&format!("{}/api/generate", self.ollama_url))
                .json(&serde_json::json!({
                    "model": model,
                    "prompt": "Describe what you see on this screen in 1 sentence. Focus on UI elements, errors, or user actions.",
                    "images": [base64_image],
                    "stream": false
                }))
                .timeout(Duration::from_secs(10))
                .send()
                .await
                .map_err(|e| format!("Request failed: {}", e))?
                .json::<serde_json::Value>()
                .await
                .map_err(|e| format!("Parse failed: {}", e))?;

            let analysis = response["response"].as_str().unwrap_or("No response").to_string();
            
            let mut state = self.state.write().await;
            state.last_analysis = analysis.clone();
            state.last_update = start.elapsed().as_millis() as u64;
            state.frames_captured += 1;
            
            Ok(analysis)
        } else {
            Ok("No emulator window found".to_string())
        }
    }

    pub async fn get_last_analysis(&self) -> String {
        self.state.read().await.last_analysis.clone()
    }

    pub async fn switch_to_cloud(&self) {
        *self.use_cloud.write().await = true;
    }

    pub async fn switch_to_local(&self) {
        *self.use_cloud.write().await = false;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS
// ═══════════════════════════════════════════════════════════════════════════

use std::sync::OnceLock;
static HADES_VISION: OnceLock<Arc<HadesVision>> = OnceLock::new();

pub fn init_hades_vision(_target_fps: u32, ollama_url: &str, local_model: &str, cloud_model: &str, use_cloud: bool) {
    let _ = HADES_VISION.set(Arc::new(HadesVision::new(ollama_url, local_model, cloud_model, use_cloud)));
}

#[tauri::command]
pub async fn hades_vision_get_current_view() -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        #[cfg(target_os = "windows")] { 
            vision.capture_and_analyze().await 
        }
        #[cfg(not(target_os = "windows"))] { 
            Ok("Vision only on Windows".to_string()) 
        }
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub async fn hades_vision_get_temporal_analysis(_frame_count: usize) -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        Ok(vision.get_last_analysis().await)
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub async fn hades_vision_switch_to_cloud() {
    if let Some(v) = HADES_VISION.get() { 
        v.switch_to_cloud().await; 
    }
}

#[tauri::command]
pub async fn hades_vision_switch_to_local() {
    if let Some(v) = HADES_VISION.get() { 
        v.switch_to_local().await; 
    }
}
