//! iPhone Emulator Integration
//! Launches Flutter Windows app
//! Window embedding requires separate native window management

use std::process::{Command, Stdio, Child};
use std::sync::Mutex;
use std::time::Duration;
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

        if process_lock.is_some() {
            return Ok("iPhone emulator already running".to_string());
        }

        let flutter_path = std::path::Path::new(&project_path);
        
        if !flutter_path.exists() {
            return Err(format!("Project path not found: {}", project_path));
        }

        // Find Flutter executable
        let flutter_exe = self.find_flutter_executable()?;
        println!("[iPhone] Using Flutter: {}", flutter_exe);

        // Launch Flutter as native Windows app
        let child = Command::new(&flutter_exe)
            .args(&["run", "-d", "windows"])
            .current_dir(flutter_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to launch Flutter: {}", e))?;

        *process_lock = Some(child);

        // Wait for Flutter window to be created
        std::thread::sleep(Duration::from_secs(3));

        // Window embedding requires native window management
        // Flutter Windows app creates its own window
        // For IDE integration, we show status in panel
        // User sees native Flutter window on taskbar

        Ok("iPhone emulator launched".to_string())
    }

    /// Find Flutter executable
    fn find_flutter_executable(&self) -> Result<String, String> {
        // Try FLUTTER_ROOT
        if let Ok(root) = std::env::var("FLUTTER_ROOT") {
            return Ok(format!("{}\\bin\\flutter.bat", root));
        }

        // Try common paths
        let paths = vec![
            r"C:\Program Files\flutter\bin\flutter.bat",
            r"C:\src\flutter\bin\flutter.bat",
            r"C:\flutter\bin\flutter.bat",
        ];

        for path in paths {
            if std::path::Path::new(path).exists() {
                return Ok(path.to_string());
            }
        }

        // Fallback to PATH
        Ok("flutter".to_string())
    }

    /// Stop the iPhone emulator
    pub fn stop(&self) -> Result<String, String> {
        let mut process_lock = self.process.lock().map_err(|e| e.to_string())?;

        if let Some(mut child) = process_lock.take() {
            child.kill().map_err(|e| format!("Failed to stop emulator: {}", e))?;
            Ok("iPhone emulator stopped".to_string())
        } else {
            Ok("iPhone emulator not running".to_string())
        }
    }

    /// Check if emulator is running
    pub fn is_running(&self) -> bool {
        self.process.lock().map(|p| p.is_some()).unwrap_or(false)
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
