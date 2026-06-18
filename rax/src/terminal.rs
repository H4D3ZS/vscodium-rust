//! Host terminal raw-mode management for the interactive serial console.
//!
//! When rax' stdin is a TTY we put it into a *raw-ish* mode so the guest serial
//! console behaves like a real one:
//!   - input is delivered a keystroke at a time (no line buffering / ICANON),
//!   - the host does not echo (the guest's own line discipline echoes back over
//!     the serial TX, which we print — echoing on the host too would double it),
//!   - control characters (Ctrl-C, Ctrl-Z, Ctrl-\, ...) are delivered to the
//!     guest as bytes instead of generating host signals (ISIG off), so they
//!     reach the guest shell,
//!   - CR/NL are passed through untranslated on input so the guest sees exactly
//!     what the terminal sent.
//!
//! Output processing (OPOST/ONLCR) is deliberately left ENABLED so a lone `\n`
//! from the guest is still expanded to `\r\n` on the host — otherwise console
//! output would "staircase".
//!
//! Because the release profile builds with `panic = "abort"` (no unwinding),
//! a stack `Drop` guard is not sufficient on its own to restore the terminal
//! when something goes wrong. We therefore ALSO:
//!   - install a panic hook that restores the terminal before printing, and
//!   - install signal handlers (SIGINT/TERM/HUP/QUIT and the fatal
//!     SIGSEGV/ABRT/BUS/ILL/FPE) that restore the terminal and then re-raise
//!     the signal with its default disposition.
//!
//! The saved termios is kept in a signal-safe static (written once before any
//! handler is installed) so the handlers — which may only call async-signal-safe
//! functions — can restore it with a bare `tcsetattr` syscall.

#[cfg(unix)]
use std::sync::atomic::{AtomicBool, Ordering};

/// Whether the terminal is currently in raw mode (and therefore needs
/// restoring). Set once when raw mode is enabled.
#[cfg(unix)]
static TTY_RAW: AtomicBool = AtomicBool::new(false);

/// Saved original terminal settings, captured before switching to raw mode.
/// Written exactly once (before handlers are installed) and only read
/// afterwards, which makes the unsynchronized access safe for signal handlers.
#[cfg(unix)]
static mut SAVED_TERMIOS: Option<libc::termios> = None;

/// RAII guard: enabling raw mode returns one of these; dropping it (on normal
/// return) restores the terminal. The panic hook and signal handlers are the
/// backstop for abnormal exits.
#[cfg(unix)]
pub struct RawTty {
    /// True if this guard actually changed the terminal (stdin was a TTY).
    active: bool,
}

#[cfg(unix)]
impl RawTty {
    /// Put stdin into raw-ish mode if it is a TTY. On a non-TTY (pipe, file,
    /// CI) this is a no-op and the returned guard does nothing.
    pub fn enable() -> Self {
        // Not a terminal — nothing to do (piped stdin, tests, redirected input).
        if unsafe { libc::isatty(libc::STDIN_FILENO) } != 1 {
            return RawTty { active: false };
        }

        // Capture the current settings so we can restore them exactly.
        let mut orig: libc::termios = unsafe { std::mem::zeroed() };
        if unsafe { libc::tcgetattr(libc::STDIN_FILENO, &mut orig) } != 0 {
            // Could not query the terminal — leave it alone.
            return RawTty { active: false };
        }

        // Stash the original BEFORE installing handlers, so a handler firing
        // immediately afterwards always sees a valid value to restore.
        unsafe {
            SAVED_TERMIOS = Some(orig);
        }

        // Build the raw-ish settings from a copy of the original.
        let mut raw = orig;
        // Local flags: char-at-a-time, no echo, deliver signals-as-bytes.
        raw.c_lflag &=
            !(libc::ICANON | libc::ECHO | libc::ECHOE | libc::ECHONL | libc::ISIG | libc::IEXTEN);
        // Input flags: pass bytes through untranslated; no host flow control.
        raw.c_iflag &= !(libc::IXON
            | libc::ICRNL
            | libc::INLCR
            | libc::IGNCR
            | libc::BRKINT
            | libc::ISTRIP
            | libc::INPCK
            | libc::PARMRK);
        // Output flags are intentionally left as-is (OPOST/ONLCR on) so guest
        // `\n` still becomes `\r\n` on the host and output does not staircase.
        // Blocking single-byte reads: VMIN=1, VTIME=0.
        raw.c_cc[libc::VMIN] = 1;
        raw.c_cc[libc::VTIME] = 0;

        if unsafe { libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, &raw) } != 0 {
            unsafe {
                SAVED_TERMIOS = None;
            }
            return RawTty { active: false };
        }

        TTY_RAW.store(true, Ordering::SeqCst);
        install_restore_hooks();

        RawTty { active: true }
    }
}

#[cfg(unix)]
impl Drop for RawTty {
    fn drop(&mut self) {
        if self.active {
            restore_terminal();
        }
    }
}

/// Restore the terminal to its saved settings. Idempotent and async-signal-safe:
/// it only reads a static and issues a single `tcsetattr` syscall.
#[cfg(unix)]
pub fn restore_terminal() {
    if !TTY_RAW.swap(false, Ordering::SeqCst) {
        return;
    }
    unsafe {
        // Raw pointer read avoids forming a reference to the mutable static.
        let saved_ptr = std::ptr::addr_of!(SAVED_TERMIOS);
        if let Some(ref termios) = *saved_ptr {
            libc::tcsetattr(libc::STDIN_FILENO, libc::TCSANOW, termios);
        }
    }
}

/// Install the panic hook + signal handlers that restore the terminal on an
/// abnormal exit. Installed once, when raw mode is first enabled.
#[cfg(unix)]
fn install_restore_hooks() {
    use std::sync::Once;
    static HOOKS: Once = Once::new();
    HOOKS.call_once(|| {
        // Panic hook: restore the terminal (so the message prints cleanly with
        // proper CR/LF and the user's shell is usable afterwards), then run the
        // default hook. With panic=abort this fires before the process aborts.
        let default_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            restore_terminal();
            default_hook(info);
        }));

        // Signal handlers. Terminating signals first.
        for &sig in &[
            libc::SIGINT,
            libc::SIGTERM,
            libc::SIGHUP,
            libc::SIGQUIT,
            libc::SIGSEGV,
            libc::SIGABRT,
            libc::SIGBUS,
            libc::SIGILL,
            libc::SIGFPE,
        ] {
            unsafe {
                libc::signal(sig, handle_fatal_signal as libc::sighandler_t);
            }
        }
    });
}

/// Async-signal-safe handler: restore the terminal, then re-raise the signal
/// with its default disposition so the normal termination/core-dump still
/// happens and the exit status reflects the signal.
#[cfg(unix)]
extern "C" fn handle_fatal_signal(sig: libc::c_int) {
    restore_terminal();
    unsafe {
        libc::signal(sig, libc::SIG_DFL);
        libc::raise(sig);
    }
}

// -----------------------------------------------------------------------------
// Non-unix fallback: no terminal control available; everything is a no-op.
// -----------------------------------------------------------------------------

#[cfg(not(unix))]
pub struct RawTty;

#[cfg(not(unix))]
impl RawTty {
    pub fn enable() -> Self {
        RawTty
    }
}

#[cfg(not(unix))]
pub fn restore_terminal() {}
