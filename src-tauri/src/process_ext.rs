use std::process::Command;

pub trait CommandExtHidden {
    fn hidden(&mut self) -> &mut Self;
}

impl CommandExtHidden for Command {
    fn hidden(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            self.creation_flags(CREATE_NO_WINDOW);
        }
        self
    }
}
