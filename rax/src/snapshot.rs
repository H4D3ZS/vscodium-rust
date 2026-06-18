//! Full VM checkpoint (save / restore / resume).
//!
//! A checkpoint is a *self-contained* image of the running machine, written to
//! a `.rxc` file. It captures everything needed to bring the machine back up
//! exactly where it left off, on a host that no longer has the original kernel
//! image:
//!   - the embedded [`CheckpointConfig`] (arch, memory size, vcpus, cmdline …)
//!     so `rax --checkpoint file.rxc` can rebuild the machine with no other
//!     flags (and the user may override any of it),
//!   - the full CPU register file + emulator-specific state (lazy flags, FPU,
//!     kernel_gs_base, PKRU, halted),
//!   - the complete writable guest RAM (zstd compressed) — which also contains
//!     the kernel/initrd that were loaded into it, so read-only images need not
//!     be shipped separately,
//!   - all stateful device models (PIC, PIT, UART, Local APIC).
//!
//! The format is intentionally versioned and magic-tagged; older `.snap` files
//! (CPU + memory only, no devices/config) are a different magic and are
//! rejected with a clear error.

use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::CheckpointConfig;
use crate::cpu::state::CpuState;
use crate::devices::lapic::LocalApic;
use crate::devices::pic::DualPic;
use crate::devices::pit::Pit;
use crate::devices::serial::Serial16550;
use crate::error::{Error, Result};

/// Magic number for checkpoint files: "RAXCKPT\0"
const CHECKPOINT_MAGIC: [u8; 8] = *b"RAXCKPT\0";

/// Current checkpoint format version. Bumped from the legacy version-1 `.snap`
/// format (CPU + memory only) to add embedded config + device state.
const CHECKPOINT_VERSION: u32 = 2;

/// Canonical checkpoint file extension ("RaX Checkpoint").
pub const CHECKPOINT_EXT: &str = "rxc";

/// Default checkpoint filename, relative to the working directory, used when no
/// `--snapshot-out` path is given.
pub const DEFAULT_CHECKPOINT_FILE: &str = "checkpoint.rxc";

/// x87 FPU state for snapshots
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FpuSnapshot {
    pub control_word: u16,
    pub status_word: u16,
    pub tag_word: u16,
    pub data_ptr: u64,
    pub instr_ptr: u64,
    pub last_opcode: u16,
    pub st: [f64; 8],
    pub top: u8,
}

impl Default for FpuSnapshot {
    fn default() -> Self {
        FpuSnapshot {
            control_word: 0x037F,
            status_word: 0,
            tag_word: 0xFFFF,
            data_ptr: 0,
            instr_ptr: 0,
            last_opcode: 0,
            st: [0.0; 8],
            top: 0,
        }
    }
}

/// Lazy flags state for snapshots
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LazyFlagsSnapshot {
    /// 0=None, 1=Add, 2=Sub, 3=Logic, 4=Inc, 5=Dec
    pub op: u8,
    pub result: u64,
    pub src: u64,
    pub dst: u64,
    pub size: u8,
}

impl Default for LazyFlagsSnapshot {
    fn default() -> Self {
        LazyFlagsSnapshot {
            op: 0, // None
            result: 0,
            src: 0,
            dst: 0,
            size: 4,
        }
    }
}

/// Extended CPU state specific to the emulator
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EmulatorState {
    pub fpu: FpuSnapshot,
    pub lazy_flags: LazyFlagsSnapshot,
    pub kernel_gs_base: u64,
    pub pkru: u32,
    pub halted: bool,
}

/// Snapshot of all stateful device models. Devices implement `Serialize` /
/// `Deserialize` directly; transient host-only fields (e.g. the LAPIC's
/// wall-clock `Instant`) are `#[serde(skip)]` and re-derived on restore.
#[derive(Clone, Serialize, Deserialize)]
pub struct DeviceState {
    /// Dual i8259 PIC (master + slave): IRR/ISR/IMR, init state, priority.
    pub pic: DualPic,
    /// i8254 PIT: all three channels, modes, reload values, timing refs.
    pub pit: Pit,
    /// 16550 UART: registers, FIFOs, staged input, interrupt state.
    pub serial: Serial16550,
    /// Local APIC: registers, LVT, timer counts, pending IPI/NMI.
    pub lapic: LocalApic,
}

/// Complete, self-contained VM checkpoint.
#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    /// Checkpoint format version
    pub version: u32,
    /// The machine-defining config, so the checkpoint resumes self-contained.
    pub config: CheckpointConfig,
    /// Instruction count when the checkpoint was taken
    pub instruction_count: u64,
    /// Wall-clock nanoseconds elapsed (`timing::elapsed_nanos`) at capture, so
    /// the resumed machine's real-time TSC/timers continue monotonically.
    pub elapsed_nanos: u64,
    /// CPU state (GPRs, segment/control/debug regs, SIMD, …)
    pub cpu_state: CpuState,
    /// Extended emulator state (lazy flags, FPU, kernel_gs_base, PKRU, halted)
    pub emulator_state: EmulatorState,
    /// Device state (PIC/PIT/UART/LAPIC)
    pub devices: DeviceState,
    /// Reported guest memory size in bytes (uncompressed)
    pub memory_size: u64,
    /// Compressed guest RAM (zstd)
    pub memory_data: Vec<u8>,
}

impl Snapshot {
    /// Create a new checkpoint from current VM state.
    pub fn new(
        config: CheckpointConfig,
        instruction_count: u64,
        elapsed_nanos: u64,
        cpu_state: CpuState,
        emulator_state: EmulatorState,
        devices: DeviceState,
        memory: &[u8],
    ) -> Result<Self> {
        // Compress memory with zstd (level 3 for a good speed/ratio balance;
        // mostly-zero guest RAM compresses extremely well).
        let compressed = zstd::encode_all(memory, 3)
            .map_err(|e| Error::Emulator(format!("Failed to compress memory: {}", e)))?;

        Ok(Snapshot {
            version: CHECKPOINT_VERSION,
            config,
            instruction_count,
            elapsed_nanos,
            cpu_state,
            emulator_state,
            devices,
            memory_size: memory.len() as u64,
            memory_data: compressed,
        })
    }

    /// Save snapshot to a file
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let file = File::create(path.as_ref())
            .map_err(|e| Error::Emulator(format!("Failed to create snapshot file: {}", e)))?;
        let mut writer = BufWriter::new(file);

        // Write magic number
        writer
            .write_all(&CHECKPOINT_MAGIC)
            .map_err(|e| Error::Emulator(format!("Failed to write snapshot magic: {}", e)))?;

        // Serialize snapshot with bincode
        bincode::serialize_into(&mut writer, self)
            .map_err(|e| Error::Emulator(format!("Failed to serialize snapshot: {}", e)))?;

        writer
            .flush()
            .map_err(|e| Error::Emulator(format!("Failed to flush snapshot: {}", e)))?;

        Ok(())
    }

    /// Load snapshot from a file
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref())
            .map_err(|e| Error::Emulator(format!("Failed to open snapshot file: {}", e)))?;
        let mut reader = BufReader::new(file);

        // Verify magic number
        let mut magic = [0u8; 8];
        reader
            .read_exact(&mut magic)
            .map_err(|e| Error::Emulator(format!("Failed to read snapshot magic: {}", e)))?;

        if magic != CHECKPOINT_MAGIC {
            // Reject the legacy "RAXSNAP\0" (CPU+memory only) format and anything
            // else with a clear, specific message.
            if &magic == b"RAXSNAP\0" {
                return Err(Error::Emulator(
                    "this is a legacy .snap snapshot (no device/config state); \
                     it cannot be resumed as a full checkpoint"
                        .to_string(),
                ));
            }
            return Err(Error::Emulator(
                "not a rax checkpoint file (bad magic)".to_string(),
            ));
        }

        // Deserialize snapshot
        let snapshot: Snapshot = bincode::deserialize_from(&mut reader)
            .map_err(|e| Error::Emulator(format!("Failed to deserialize checkpoint: {}", e)))?;

        if snapshot.version != CHECKPOINT_VERSION {
            return Err(Error::Emulator(format!(
                "Unsupported checkpoint version {} (expected {})",
                snapshot.version, CHECKPOINT_VERSION
            )));
        }

        Ok(snapshot)
    }

    /// Decompress and return memory contents
    pub fn decompress_memory(&self) -> Result<Vec<u8>> {
        let mut decompressed = Vec::with_capacity(self.memory_size as usize);
        zstd::stream::copy_decode(&self.memory_data[..], &mut decompressed)
            .map_err(|e| Error::Emulator(format!("Failed to decompress memory: {}", e)))?;

        if decompressed.len() != self.memory_size as usize {
            return Err(Error::Emulator(format!(
                "Memory size mismatch: expected {} bytes, got {}",
                self.memory_size,
                decompressed.len()
            )));
        }

        Ok(decompressed)
    }

    /// Get a summary string for display
    pub fn summary(&self) -> String {
        let mem_mb = self.memory_size / (1024 * 1024);
        let compressed_mb = self.memory_data.len() / (1024 * 1024);
        let ratio = if self.memory_data.is_empty() {
            0.0
        } else {
            self.memory_size as f64 / self.memory_data.len() as f64
        };

        format!(
            "checkpoint @ insn #{}: {:?} {}vCPU, {}MB RAM ({}MB compressed, {:.1}x), devices+config embedded",
            self.instruction_count,
            self.config.arch,
            self.config.vcpus,
            mem_mb,
            compressed_mb,
            ratio
        )
    }
}

/// Configuration for automatic snapshotting
#[derive(Clone, Debug)]
pub struct SnapshotConfig {
    /// Take snapshot every N instructions (0 = disabled)
    pub interval: u64,
    /// Take snapshot at specific instruction counts
    pub at_instructions: Vec<u64>,
    /// Directory to save snapshots
    pub output_dir: String,
    /// Prefix for snapshot filenames
    pub prefix: String,
}

impl Default for SnapshotConfig {
    fn default() -> Self {
        SnapshotConfig {
            interval: 0,
            at_instructions: Vec::new(),
            output_dir: ".".to_string(),
            prefix: "snapshot".to_string(),
        }
    }
}

impl SnapshotConfig {
    /// Check if a snapshot should be taken at the given instruction count
    pub fn should_snapshot(&self, insn_count: u64) -> bool {
        // Check interval
        if self.interval > 0 && insn_count > 0 && insn_count % self.interval == 0 {
            return true;
        }
        // Check specific instruction counts
        self.at_instructions.contains(&insn_count)
    }

    /// Generate filename for a snapshot at the given instruction count
    pub fn filename(&self, insn_count: u64) -> String {
        format!(
            "{}/{}_{:012}.{}",
            self.output_dir, self.prefix, insn_count, CHECKPOINT_EXT
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn device_state_bincode_roundtrip() {
        crate::timing::init();
        let devices = DeviceState {
            pic: DualPic::new(),
            pit: Pit::new(),
            serial: Serial16550::new(0x3f8),
            lapic: LocalApic::new(0),
        };
        let bytes = bincode::serialize(&devices).expect("serialize device state");
        let back: DeviceState = bincode::deserialize(&bytes).expect("deserialize device state");
        // Re-serializing the round-tripped value must reproduce the same bytes:
        // proves every field (and the #[serde(skip)] LAPIC Instant) is stable.
        let bytes2 = bincode::serialize(&back).expect("re-serialize device state");
        assert_eq!(bytes, bytes2);
    }

    #[test]
    fn checkpoint_config_bincode_roundtrip() {
        use crate::config::{ArchKind, BackendKind, CheckpointConfig, Endianness, HexagonIsa};
        let cp = CheckpointConfig {
            arch: ArchKind::X86_64,
            backend: BackendKind::Emulator,
            memory_bytes: 512 << 20,
            vcpus: 1,
            kernel: std::path::PathBuf::from("/some/vmlinux"),
            initrd: Some(std::path::PathBuf::from("/some/initrd.cpio")),
            cmdline: "console=ttyS0 root=/dev/ram0".to_string(),
            hexagon_isa: HexagonIsa::V68,
            hexagon_endian: Endianness::Little,
            hexagon_entry: None,
            hexagon_load_addr: None,
            aarch64_isa: Default::default(),
            aarch32_isa: Default::default(),
            cortexm_isa: Default::default(),
            cortexr_isa: Default::default(),
            arm_entry: None,
            arm_load_addr: None,
            arm_dtb: None,
        };
        let bytes = bincode::serialize(&cp).expect("serialize checkpoint config");
        let back: CheckpointConfig = bincode::deserialize(&bytes).expect("deserialize");
        assert_eq!(back.memory_bytes, cp.memory_bytes);
        assert_eq!(back.cmdline, cp.cmdline);
        assert_eq!(back.arch, cp.arch);
        assert_eq!(back.backend, cp.backend);
    }

    #[test]
    fn test_snapshot_config() {
        let mut config = SnapshotConfig::default();
        config.interval = 1000;
        config.at_instructions = vec![500, 1500];

        assert!(!config.should_snapshot(0));
        assert!(config.should_snapshot(500));
        assert!(config.should_snapshot(1000));
        assert!(config.should_snapshot(1500));
        assert!(config.should_snapshot(2000));
        assert!(!config.should_snapshot(1234));
    }
}
