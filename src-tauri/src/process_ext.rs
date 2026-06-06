use std::process::Command;

#[cfg(target_os = "windows")]
pub const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub trait CommandExtHidden {
    fn hidden(&mut self) -> &mut Self;
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
}

pub trait TokioCommandExtHidden {
    fn hidden(&mut self) -> &mut Self;
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
}

/// Build a std::process::Command with CREATE_NO_WINDOW on Windows.
pub fn hidden_command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    cmd.hidden();
    cmd
}
