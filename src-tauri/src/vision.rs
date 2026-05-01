//! AIRI Vision Module
//! Real-time screen analysis using Qwen2.5-VL for mobile QA

use reqwest::Client;
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose, Engine as _};
use image::ImageFormat;
use std::io::Cursor;

#[derive(Serialize)]
struct OllamaMessage {
    role: String,
    content: String,
    images: Vec<String>,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct VisionAnalysisResult {
    pub status: String,
    pub error_message: Option<String>,
    pub ui_elements: Vec<String>,
    pub suggested_action: String,
    pub raw_analysis: String,
}

/// Capture screen and analyze with Qwen2.5-VL or other vision model
pub async fn analyze_screen(
    prompt: &str,
    screenshot_bytes: &[u8],
    ollama_url: &str,
    model: &str,
) -> Result<VisionAnalysisResult, Box<dyn std::error::Error>> {
    // 1. Resize Image for Speed (Max 1024px width/height)
    let img = image::load_from_memory(screenshot_bytes)?;
    let resized_img = img.resize(1024, 1024, image::imageops::FilterType::Lanczos3);
    
    // 2. Convert to PNG Bytes
    let mut buffer = Vec::new();
    resized_img.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)?;
    
    // 3. Encode to Base64
    let base64_image = general_purpose::STANDARD.encode(&buffer);

    // 4. Prepare Vision Analysis Prompt
    let vision_prompt = format!(r#"Act as a Senior Mobile QA Engineer. Analyze the attached screenshot of an Android/iOS emulator.

1. Identify any visible error messages, red text, or crash dialogs.
2. List all interactive elements (buttons, inputs) and their state (enabled/disabled).
3. If there is an error, explain the likely cause based on the UI context.
4. Output in JSON format:
{{
  "status": "healthy" | "error",
  "error_message": "string or null",
  "ui_elements": ["Button: Submit", "Input: Email"],
  "suggested_action": "string"
}}

User's specific request: {}"#, prompt);

    // 5. Prepare Payload
    let request_body = OllamaRequest {
        model: model.to_string(),
        messages: vec![
            OllamaMessage {
                role: "user".to_string(),
                content: vision_prompt,
                images: vec![base64_image],
            }
        ],
        stream: false,
    };

    // 6. Send to Ollama (via SSH Tunnel or Local)
    let client = Client::new();
    let res = client
        .post(&format!("{}/api/chat", ollama_url))
        .json(&request_body)
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("Ollama API error: {}", res.status()).into());
    }

    let response: OllamaResponse = res.json().await?;
    
    // 7. Parse JSON response (extract from markdown code blocks if present)
    let raw_content = &response.message.content;
    let json_content = extract_json_from_response(raw_content);
    
    // 8. Parse into structured result
    let analysis_result = parse_vision_result(&json_content, raw_content);
    
    Ok(analysis_result)
}

/// Extract JSON from markdown code blocks
fn extract_json_from_response(content: &str) -> String {
    // Try to find JSON between ```json and ```
    if let Some(start) = content.find("```json") {
        let start = start + 7;
        if let Some(end) = content[start..].find("```") {
            return content[start..start + end].trim().to_string();
        }
    }
    
    // Try to find JSON between ``` and ```
    if let Some(start) = content.find("```") {
        let start = start + 3;
        if let Some(end) = content[start..].find("```") {
            return content[start..start + end].trim().to_string();
        }
    }
    
    // Return as-is if no code blocks
    content.trim().to_string()
}

/// Parse vision analysis result from JSON or text
fn parse_vision_result(json_content: &str, raw_content: &str) -> VisionAnalysisResult {
    // Try to parse as JSON
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(json_content) {
        let status = parsed.get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        
        let error_message = parsed.get("error_message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        let ui_elements = parsed.get("ui_elements")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect())
            .unwrap_or_default();
        
        let suggested_action = parsed.get("suggested_action")
            .and_then(|v| v.as_str())
            .unwrap_or("No specific action suggested")
            .to_string();
        
        return VisionAnalysisResult {
            status,
            error_message,
            ui_elements,
            suggested_action,
            raw_analysis: raw_content.to_string(),
        };
    }
    
    // Fallback: create result from raw text
    let status = if raw_content.to_lowercase().contains("error") {
        "error".to_string()
    } else {
        "healthy".to_string()
    };
    
    VisionAnalysisResult {
        status,
        error_message: None,
        ui_elements: vec![],
        suggested_action: "Review the analysis above".to_string(),
        raw_analysis: raw_content.to_string(),
    }
}

/// Capture entire screen
pub fn capture_screen() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let screen = screenshots::Screen::from_point(0, 0)?;
        let buffer = screen.capture()?;
        
        let img = image::RgbaImage::from_raw(
            buffer.width(),
            buffer.height(),
            buffer.into_raw(),
        ).ok_or("Failed to create image from buffer")?;
        
        let mut png_bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut png_bytes), ImageFormat::Png)?;
        return Ok(png_bytes);
    }
    
    Err("Screen capture not supported on this platform".into())
}

/// Capture specific window by title (not implemented yet)
pub fn capture_window(_window_title: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // For now, capture full screen
    capture_screen()
}

/// Tauri command: Capture screen
#[tauri::command]
#[allow(dead_code)]
pub async fn airi_vision_capture_screen() -> Result<Vec<u8>, String> {
    capture_screen().map_err(|e| e.to_string())
}

/// Tauri command: Analyze screen with AI vision
#[tauri::command]
pub async fn airi_vision_analyze_screen(
    prompt: String,
    ollama_url: String,
    model: Option<String>,
) -> Result<VisionAnalysisResult, String> {
    // Capture screen
    let screenshot = capture_screen().map_err(|e| format!("Capture failed: {}", e))?;
    
    // Use provided model or default to moondream:1.8b (good for 8GB VRAM)
    let vision_model = model.unwrap_or_else(|| "moondream:1.8b".to_string());
    
    // Analyze with vision model
    analyze_screen(&prompt, &screenshot, &ollama_url, &vision_model)
        .await
        .map_err(|e| format!("Analysis failed: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_response() {
        let markdown = r#"```json
{"status": "healthy", "ui_elements": ["Button: Submit"]}
```"#;
        let result = extract_json_from_response(markdown);
        assert!(result.contains("status"));
        assert!(result.contains("healthy"));
    }
}
