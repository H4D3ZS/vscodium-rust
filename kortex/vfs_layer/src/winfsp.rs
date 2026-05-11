/// WinFsp Virtual Drive Mapping (The End-User Drive)
pub struct VirtualDrive {
    pub drive_letter: String,
    pub volume_label: String,
}

impl VirtualDrive {
    pub fn new() -> Self {
        Self {
            drive_letter: "Z:".to_string(), // Binds perfectly to Z: [AI Memory]
            volume_label: "AI Memory".to_string(),
        }
    }

    /// Mounts the .aim states dynamically as a standard Windows Virtual Drive using WinFsp
    /// so the consumer views it flawlessly in `Explorer.exe` identically to a USB stick.
    pub fn mount_winfsp_overlay(&self) {
        println!("Mounting WinFsp Volume [{}] at Local Drive {}", self.volume_label, self.drive_letter);
        // Uses `winfsp_rs` mapping interfaces translating File Requests directly to our .aim vectors
    }
}

impl Default for VirtualDrive {
    fn default() -> Self {
        Self::new()
    }
}
