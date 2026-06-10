use std::process::Command;

#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;
#[cfg(target_os = "windows")]
pub const CREATE_NEW_PROCESS_GROUP: u32 = 0x0000_0200;
#[cfg(target_os = "windows")]
pub const DETACHED_PROCESS: u32 = 0x0000_0008;

pub trait CommandExtHidden {
    fn hidden(&mut self) -> &mut Self;
    /// Piped sidecar (browser-agent, python) — no DETACHED_PROCESS (that can force AllocConsole).
    fn hidden_sidecar(&mut self) -> &mut Self;
    /// Detached sidecar without stdio pipes — e.g. long-lived helpers that must not inherit IDE console signals.
    fn detached_sidecar(&mut self) -> &mut Self;
}

impl CommandExtHidden for Command {
    fn hidden(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW);
        }
        self
    }

    fn hidden_sidecar(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);
        }
        self
    }

    fn detached_sidecar(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS);
        }
        self
    }
}

pub trait TokioCommandExtHidden {
    fn hidden(&mut self) -> &mut Self;
    fn hidden_sidecar(&mut self) -> &mut Self;
    fn detached_sidecar(&mut self) -> &mut Self;
}

impl TokioCommandExtHidden for tokio::process::Command {
    fn hidden(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW);
        }
        self
    }

    fn hidden_sidecar(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP);
        }
        self
    }

    fn detached_sidecar(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            self.creation_flags(CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS);
        }
        self
    }
}

/// Build a std::process::Command with CREATE_NO_WINDOW on Windows.
pub fn hidden_command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    cmd.hidden();
    cmd
}

/// Hide ConsoleWindowClass windows owned by `pid` (works when parent is a console app in `tauri dev`).
#[cfg(windows)]
pub fn hide_console_windows_for_pid(pid: u32) {
    use std::sync::atomic::{AtomicU32, Ordering};
    use windows::Win32::Foundation::{BOOL, HWND, LPARAM, TRUE};
    use windows::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetClassNameW, GetWindowThreadProcessId, ShowWindow, SW_HIDE,
    };

    static TARGET_PID: AtomicU32 = AtomicU32::new(0);
    TARGET_PID.store(pid, Ordering::SeqCst);

    unsafe extern "system" fn enum_cb(hwnd: HWND, _: LPARAM) -> BOOL {
        use std::sync::atomic::Ordering;
        let target = TARGET_PID.load(Ordering::SeqCst);
        let mut win_pid = 0u32;
        unsafe {
            GetWindowThreadProcessId(hwnd, Some(&mut win_pid));
        }
        if win_pid != target {
            return TRUE;
        }
        let mut cls = [0u16; 32];
        let n = unsafe { GetClassNameW(hwnd, &mut cls) };
        if n > 0 {
            let name = String::from_utf16_lossy(&cls[..n as usize]);
            if name == "ConsoleWindowClass" {
                let _ = unsafe { ShowWindow(hwnd, SW_HIDE) };
            }
        }
        TRUE
    }

    unsafe {
        let _ = EnumWindows(Some(enum_cb), LPARAM(0));
    }
}

#[cfg(not(windows))]
pub fn hide_console_windows_for_pid(_pid: u32) {}

/// Call right after spawning a console child — hides any flash under `tauri dev` (console parent).
pub fn suppress_child_console_after_spawn(pid: u32) {
    hide_console_windows_for_pid(pid);
    std::thread::spawn(move || {
        for _ in 0..40 {
            hide_console_windows_for_pid(pid);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    });
}
