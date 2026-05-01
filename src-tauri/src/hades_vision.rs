//! HADES Real-Time Vision System
//! "The Eye of Hades" - Continuous framebuffer streaming with ROI focus

use std::sync::Arc;
use tokio::sync::RwLock;
use image::{DynamicImage, ImageBuffer, Rgba};

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HWND, RECT, LPARAM},
    Graphics::Gdi::{BitBlt, CreateCompatibleDC, CreateDIBSection, DeleteDC, GetDC, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, DIB_RGB_COLORS, SRCCOPY},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowTextA, GetWindowRect, IsWindowVisible},
};

// ═══════════════════════════════════════════════════════════════════════════
// FRAME BUFFER (Ring Buffer for Continuous Vision)
// ═══════════════════════════════════════════════════════════════════════════

pub struct FrameBuffer {
    frames: Arc<RwLock<Vec<Frame>>>,
    max_frames: usize,
    width: u32,
    height: u32,
}

pub struct Frame {
    pub timestamp: u64,
    pub data: Vec<u8>, // RGBA pixels
    pub roi: Option<RegionOfInterest>,
}

pub struct RegionOfInterest {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub label: String, // "emulator", "terminal", "editor", etc.
}

impl FrameBuffer {
    pub fn new(width: u32, height: u32, max_frames: usize) -> Self {
        Self {
            frames: Arc::new(RwLock::new(Vec::new())),
            max_frames,
            width,
            height,
        }
    }

    pub async fn push(&self, frame: Frame) {
        let mut frames = self.frames.write().await;
        frames.push(frame);
        
        // Keep only last N frames (ring buffer)
        if frames.len() > self.max_frames {
            frames.remove(0);
        }
    }

    pub async fn get_latest(&self) -> Option<Frame> {
        let frames = self.frames.read().await;
        frames.last().cloned()
    }

    pub async fn get_sequence(&self, count: usize) -> Vec<Frame> {
        let frames = self.frames.read().await;
        let start = frames.len().saturating_sub(count);
        frames[start..].to_vec()
    }

    pub async fn clear(&self) {
        let mut frames = self.frames.write().await;
        frames.clear();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WINDOW DETECTION (Find Emulator/Target Windows)
// ═══════════════════════════════════════════════════════════════════════════

pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub rect: RECT,
}

#[cfg(target_os = "windows")]
pub fn find_emulator_window() -> Option<WindowInfo> {
    use std::ffi::CString;
    use windows::Win32::UI::WindowsAndMessaging::FindWindowA;

    // Common emulator window titles
    let emulator_titles = vec![
        "Android Emulator",
        "Pixel",
        "iPhone Simulator",
        "Virtual iPhone",
        "Acheron",
        "scrcpy",
    ];

    for title in emulator_titles {
        if let Ok(c_title) = CString::new(title) {
            unsafe {
                let hwnd = FindWindowA(None, c_title.as_ptr());
                if hwnd.0 != 0 {
                    let mut rect = RECT::default();
                    if GetWindowRect(hwnd, &mut rect).is_ok() {
                        return Some(WindowInfo {
                            hwnd: hwnd.0,
                            title: title.to_string(),
                            rect,
                        });
                    }
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
pub fn capture_window_roi(hwnd: isize, x: i32, y: i32, w: u32, h: u32) -> Option<Vec<u8>> {
    use std::mem;
    use windows::Win32::Graphics::Gdi::{DeleteObject, GetObjectA};

    unsafe {
        let hdc_screen = GetDC(HWND(0));
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        
        // Create DIB section
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                biHeight: -(h as i32), // Top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0, // BI_RGB
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
        if hbitmap.is_err() {
            DeleteDC(hdc_mem);
            ReleaseDC(HWND(0), hdc_screen);
            return None;
        }
        let hbitmap = hbitmap.unwrap();

        SelectObject(hdc_mem, hbitmap);

        // BitBlt from screen to memory DC
        BitBlt(
            hdc_mem,
            0,
            0,
            w as i32,
            h as i32,
            hdc_screen,
            x,
            y,
            SRCCOPY,
        ).ok()?;

        // Copy bits to Vec
        let size = (w * h * 4) as usize;
        let mut rgba = Vec::with_capacity(size);
        std::ptr::copy_nonoverlapping(bits, rgba.as_mut_ptr() as *mut _, size);
        rgba.set_len(size);

        // Cleanup
        DeleteObject(hbitmap);
        DeleteDC(hdc_mem);
        ReleaseDC(HWND(0), hdc_screen);

        Some(rgba)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SMART CAPTURE (ROI-focused, performance optimized)
// ═══════════════════════════════════════════════════════════════════════════

pub struct SmartCapturer {
    frame_buffer: FrameBuffer,
    target_fps: u32,
    roi_detection_interval: u64, // ms
}

impl SmartCapturer {
    pub fn new(target_fps: u32, roi_size: (u32, u32)) -> Self {
        Self {
            frame_buffer: FrameBuffer::new(roi_size.0, roi_size.1, 10),
            target_fps,
            roi_detection_interval: 5000, // Detect ROI every 5 seconds
        }
    }

    #[cfg(target_os = "windows")]
    pub async fn start_capture_loop(&self) {
        use tokio::time::{Duration, Instant};
        
        let mut last_roi_check = Instant::now();
        let mut current_roi: Option<RegionOfInterest> = None;
        let frame_delay = Duration::from_millis(1000 / self.target_fps as u64);

        loop {
            // Update ROI periodically
            if last_roi_check.elapsed().as_millis() > self.roi_detection_interval as u128 {
                current_roi = self.detect_best_roi().await;
                last_roi_check = Instant::now();
            }

            // Capture frame
            if let Some(roi) = &current_roi {
                if let Some(frame_data) = capture_window_roi(
                    roi.x as isize,
                    roi.y as isize,
                    roi.width,
                    roi.height,
                ) {
                    let frame = Frame {
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64,
                        data: frame_data,
                        roi: Some(roi.clone()),
                    };
                    self.frame_buffer.push(frame).await;
                }
            }

            tokio::time::sleep(frame_delay).await;
        }
    }

    async fn detect_best_roi(&self) -> Option<RegionOfInterest> {
        #[cfg(target_os = "windows")]
        {
            // Try to find emulator window
            if let Some(window) = find_emulator_window() {
                let w = (window.rect.right - window.rect.left) as u32;
                let h = (window.rect.bottom - window.rect.top) as u32;
                
                return Some(RegionOfInterest {
                    x: window.rect.left as u32,
                    y: window.rect.top as u32,
                    width: w,
                    height: h,
                    label: "emulator".to_string(),
                });
            }
        }

        // Fallback: full screen capture (center region)
        Some(RegionOfInterest {
            x: 0,
            y: 0,
            width: 1024,
            height: 1024,
            label: "screen".to_string(),
        })
    }

    pub fn get_frame_buffer(&self) -> &FrameBuffer {
        &self.frame_buffer
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// VISION ANALYSIS (Send frames to Ollama VLM)
// ═══════════════════════════════════════════════════════════════════════════

pub struct VisionAnalyzer {
    ollama_url: String,
    model: String,
    local_model: String,
    cloud_model: String,
    use_cloud: bool,
}

impl VisionAnalyzer {
    pub fn new(ollama_url: &str, local_model: &str, cloud_model: &str, use_cloud: bool) -> Self {
        Self {
            ollama_url: ollama_url.to_string(),
            model: if use_cloud { cloud_model.to_string() } else { local_model.to_string() },
            local_model: local_model.to_string(),
            cloud_model: cloud_model.to_string(),
            use_cloud,
        }
    }

    pub async fn analyze_frame(&self, frame: &Frame) -> Result<String, Box<dyn std::error::Error>> {
        // Resize frame for model (512x512 for local, 1024x1024 for cloud)
        let target_size = if self.use_cloud { 1024 } else { 512 };
        let resized = self.resize_frame(&frame.data, frame.roi.as_ref().map(|r| (r.width, r.height)).unwrap_or((1024, 1024)), target_size);
        
        // Encode to base64
        let base64_image = base64::encode(&resized);

        // Build prompt with temporal context
        let prompt = if let Some(roi) = &frame.roi {
            format!(
                "You are viewing a {} screen ({}x{}). Describe what you see in 1-2 sentences. Focus on UI elements, errors, or user actions.",
                roi.label, roi.width, roi.height
            )
        } else {
            "Describe what you see in this screen in 1-2 sentences.".to_string()
        };

        // Call Ollama
        let response = reqwest::Client::new()
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "images": [base64_image],
                "stream": false
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["response"].as_str().unwrap_or("No response").to_string())
    }

    pub async fn analyze_sequence(&self, frames: &[Frame]) -> Result<String, Box<dyn std::error::Error>> {
        // For multi-frame analysis (temporal understanding)
        if frames.is_empty() {
            return Ok("No frames to analyze".to_string());
        }

        let mut images = Vec::new();
        for frame in frames {
            let target_size = if self.use_cloud { 1024 } else { 512 };
            let resized = self.resize_frame(&frame.data, frame.roi.as_ref().map(|r| (r.width, r.height)).unwrap_or((1024, 1024)), target_size);
            images.push(base64::encode(&resized));
        }

        let prompt = format!(
            "You are viewing a sequence of {} frames from a screen. Describe what changed between frames and what the user is doing.",
            frames.len()
        );

        let response = reqwest::Client::new()
            .post(&format!("{}/api/generate", self.ollama_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "images": images,
                "stream": false
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        Ok(response["response"].as_str().unwrap_or("No response").to_string())
    }

    fn resize_frame(&self, data: &[u8], from: (u32, u32), to: u32) -> Vec<u8> {
        if let Ok(img) = image::ImageBuffer::<Rgba<u8>, _>::from_raw(from.0, from.1, data.to_vec()) {
            let dyn_img = DynamicImage::ImageRgba8(img);
            let resized = dyn_img.resize_exact(to, to, image::imageops::FilterType::Triangle);
            resized.into_rgba8().into_raw()
        } else {
            // Fallback: return original
            data.to_vec()
        }
    }

    pub fn switch_to_cloud(&mut self) {
        self.use_cloud = true;
        self.model = self.cloud_model.clone();
    }

    pub fn switch_to_local(&mut self) {
        self.use_cloud = false;
        self.model = self.local_model.clone();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HADES VISION MANAGER (Main Entry Point)
// ═══════════════════════════════════════════════════════════════════════════

pub struct HadesVision {
    capturer: SmartCapturer,
    analyzer: VisionAnalyzer,
    running: Arc<RwLock<bool>>,
}

impl HadesVision {
    pub fn new(
        target_fps: u32,
        ollama_url: &str,
        local_model: &str,
        cloud_model: &str,
        use_cloud: bool,
    ) -> Self {
        Self {
            capturer: SmartCapturer::new(target_fps, (1024, 1024)),
            analyzer: VisionAnalyzer::new(ollama_url, local_model, cloud_model, use_cloud),
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start(&self) {
        let mut running = self.running.write().await;
        *running = true;
        drop(running);

        // Start capture loop in background
        let capturer = self.capturer.clone();
        tokio::spawn(async move {
            capturer.start_capture_loop().await;
        });

        println!("[HADES Vision] Real-time capture started at {} FPS", 5);
    }

    pub async fn stop(&self) {
        let mut running = self.running.write().await;
        *running = false;
    }

    pub async fn get_current_view(&self) -> Option<String> {
        let frame_buffer = self.capturer.get_frame_buffer();
        if let Some(frame) = frame_buffer.get_latest().await {
            match self.analyzer.analyze_frame(&frame).await {
                Ok(analysis) => Some(analysis),
                Err(e) => Some(format!("Vision error: {}", e)),
            }
        } else {
            None
        }
    }

    pub async fn get_temporal_analysis(&self, frame_count: usize) -> Option<String> {
        let frame_buffer = self.capturer.get_frame_buffer();
        let frames = frame_buffer.get_sequence(frame_count).await;
        
        if frames.is_empty() {
            return None;
        }

        match self.analyzer.analyze_sequence(&frames).await {
            Ok(analysis) => Some(analysis),
            Err(e) => Some(format!("Vision error: {}", e)),
        }
    }

    pub fn switch_to_cloud(&mut self) {
        self.analyzer.switch_to_cloud();
    }

    pub fn switch_to_local(&mut self) {
        self.analyzer.switch_to_local();
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TAURI COMMANDS
// ═══════════════════════════════════════════════════════════════════════════

use std::sync::OnceLock;
static HADES_VISION: OnceLock<Arc<HadesVision>> = OnceLock::new();

pub fn init_hades_vision(
    target_fps: u32,
    ollama_url: &str,
    local_model: &str,
    cloud_model: &str,
    use_cloud: bool,
) {
    let vision = HadesVision::new(target_fps, ollama_url, local_model, cloud_model, use_cloud);
    let _ = HADES_VISION.set(Arc::new(vision));
    
    // Auto-start vision
    if let Some(v) = HADES_VISION.get() {
        let v = v.clone();
        tokio::spawn(async move {
            v.start().await;
        });
    }
}

#[tauri::command]
pub async fn hades_vision_get_current_view() -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        vision.get_current_view().await.ok_or("No view available".to_string())
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub async fn hades_vision_get_temporal_analysis(frame_count: usize) -> Result<String, String> {
    if let Some(vision) = HADES_VISION.get() {
        vision.get_temporal_analysis(frame_count).await.ok_or("No frames available".to_string())
    } else {
        Err("Vision not initialized".to_string())
    }
}

#[tauri::command]
pub fn hades_vision_switch_to_cloud() {
    if let Some(vision) = HADES_VISION.get() {
        let mut v = vision.as_ref().clone();
        v.switch_to_cloud();
    }
}

#[tauri::command]
pub fn hades_vision_switch_to_local() {
    if let Some(vision) = HADES_VISION.get() {
        let mut v = vision.as_ref().clone();
        v.switch_to_local();
    }
}
