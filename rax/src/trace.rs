//! Instruction trace output module (SDE-compatible format).
//!
//! When enabled, writes a trace of each executed instruction to a file
//! in a format compatible with Intel SDE's debugtrace output.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::sync::Mutex;

/// Global trace writer (protected by mutex for thread safety)
static TRACE_WRITER: Mutex<Option<BufWriter<File>>> = Mutex::new(None);

/// Initialize tracing with an output file path
pub fn init(path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let writer = BufWriter::with_capacity(1024 * 1024, file); // 1MB buffer
    *TRACE_WRITER.lock().unwrap() = Some(writer);
    Ok(())
}

/// Check if tracing is enabled
#[inline(always)]
pub fn is_enabled() -> bool {
    TRACE_WRITER.lock().unwrap().is_some()
}

/// Write a trace line
#[inline]
pub fn write_insn(rip: u64, disasm: &str, reg_changes: &str) {
    if let Some(ref mut writer) = *TRACE_WRITER.lock().unwrap() {
        let _ = writeln!(
            writer,
            "INS 0x{:016x}   {:<50} | {}",
            rip, disasm, reg_changes
        );
    }
}

/// Write a memory read
#[inline]
pub fn write_read(addr: u64, size: usize, value: u64) {
    if let Some(ref mut writer) = *TRACE_WRITER.lock().unwrap() {
        match size {
            1 => {
                let _ = writeln!(writer, "Read 0x{:02x} = *(UINT8*)0x{:x}", value, addr);
            }
            2 => {
                let _ = writeln!(writer, "Read 0x{:04x} = *(UINT16*)0x{:x}", value, addr);
            }
            4 => {
                let _ = writeln!(writer, "Read 0x{:08x} = *(UINT32*)0x{:x}", value, addr);
            }
            8 => {
                let _ = writeln!(writer, "Read 0x{:016x} = *(UINT64*)0x{:x}", value, addr);
            }
            _ => {
                let _ = writeln!(writer, "Read 0x{:x} = *({} bytes)0x{:x}", value, size, addr);
            }
        }
    }
}

/// Write a memory write
#[inline]
pub fn write_write(addr: u64, size: usize, value: u64) {
    if let Some(ref mut writer) = *TRACE_WRITER.lock().unwrap() {
        match size {
            1 => {
                let _ = writeln!(writer, "Write *(UINT8*)0x{:x} = 0x{:02x}", addr, value);
            }
            2 => {
                let _ = writeln!(writer, "Write *(UINT16*)0x{:x} = 0x{:04x}", addr, value);
            }
            4 => {
                let _ = writeln!(writer, "Write *(UINT32*)0x{:x} = 0x{:08x}", addr, value);
            }
            8 => {
                let _ = writeln!(writer, "Write *(UINT64*)0x{:x} = 0x{:016x}", addr, value);
            }
            _ => {
                let _ = writeln!(
                    writer,
                    "Write *({} bytes)0x{:x} = 0x{:x}",
                    size, addr, value
                );
            }
        }
    }
}

/// Write XMM register change
#[inline]
pub fn write_xmm(reg: usize, low: u64, high: u64) {
    if let Some(ref mut writer) = *TRACE_WRITER.lock().unwrap() {
        let _ = writeln!(writer, "\tXMM{} := {:016x}_{:016x}", reg, high, low);
    }
}

/// Flush the trace buffer
pub fn flush() {
    if let Some(ref mut writer) = *TRACE_WRITER.lock().unwrap() {
        let _ = writer.flush();
    }
}

/// Close the trace file
pub fn close() {
    let mut guard = TRACE_WRITER.lock().unwrap();
    if let Some(ref mut writer) = *guard {
        let _ = writer.flush();
    }
    *guard = None;
}
