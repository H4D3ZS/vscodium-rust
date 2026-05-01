//! iPhone Emulator Integration
//! Launches and manages the Virtual iPhone Emulator (Flutter)

use std::process::{Command, Stdio, Child};
use std::sync::Mutex;
use tauri::State;

/// Manages the iPhone emulator process
pub struct iPhoneEmulatorManager {
    process: Mutex<Option<Child>>,
}

impl iPhoneEmulatorManager {
    pub fn new() -> Self {
        Self {
            process: Mutex::new(None),
        }
    }

    /// Launch the Flutter iPhone Emulator
    pub fn launch(&self, project_path: String) -> Result<String, String> {
        let mut process_lock = self.process.lock().map_err(|e| e.to_string())?;

        // Check if already running
        if process_lock.is_some() {
            return Ok("iPhone emulator already running".to_string());
        }

        // Launch Flutter web server
        let flutter_path = std::path::Path::new(&project_path);
        
        if !flutter_path.exists() {
            return Err(format!("Project path not found: {}", project_path));
        }

        // Start Flutter web server
        let child = Command::new("flutter")
            .args(&["run", "-d", "web-server", "--web-port", "5175", "--web-hostname", "localhost"])
            .current_dir(flutter_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to launch Flutter: {}", e))?;

        *process_lock = Some(child);

        Ok(format!("iPhone emulator launched at http://localhost:5175"))
    }

    /// Stop the iPhone emulator
    pub fn stop(&self) -> Result<String, String> {
        let mut process_lock = self.process.lock().map_err(|e| e.to_string())?;

        if let Some(mut child) = process_lock.take() {
            // Kill the process
            child.kill().map_err(|e| format!("Failed to stop emulator: {}", e))?;
            Ok("iPhone emulator stopped".to_string())
        } else {
            Ok("iPhone emulator not running".to_string())
        }
    }

    /// Check if emulator is running
    pub fn is_running(&self) -> bool {
        if let Ok(process_lock) = self.process.lock() {
            process_lock.is_some()
        } else {
            false
        }
    }
}

/// Tauri command: Launch iPhone emulator
#[tauri::command]
pub fn launch_iphone_emulator(
    manager: State<'_, iPhoneEmulatorManager>,
    project_path: String,
) -> Result<String, String> {
    manager.launch(project_path)
}

/// Tauri command: Stop iPhone emulator
#[tauri::command]
pub fn stop_iphone_emulator(
    manager: State<'_, iPhoneEmulatorManager>,
) -> Result<String, String> {
    manager.stop()
}

/// Tauri command: Check if iPhone emulator is running
#[tauri::command]
pub fn is_iphone_emulator_running(
    manager: State<'_, iPhoneEmulatorManager>,
) -> bool {
    manager.is_running()
}
