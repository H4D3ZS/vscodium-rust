//! Intel AC'97 audio controller device model (ICH-style 82801 / "AC97").
//!
//! Classic AC'97 controllers expose two PCI **I/O** BARs, so this is modeled as
//! an [`IoDevice`] (PIO) rather than MMIO:
//!
//! * **NAM** — *Native Audio Mixer*, 256 bytes (BAR0). This is the AC'97 *codec*
//!   register file: reset/capabilities, the volume mixer registers, record
//!   select/gain, etc. Codec registers are architecturally 16 bits wide and
//!   accessed as words.
//! * **NABM** — *Native Audio Bus Master*, 64 bytes (BAR1). This is the
//!   controller's DMA engine register file: three bus-master channels (PCM In,
//!   PCM Out, Mic In), plus a global control and global status register.
//!
//! Both BARs are dispatched through one device because the [`IoDevice`] bus
//! dispatches one byte at a time over a contiguous port range; the device is
//! given the two base ports and decodes accesses to either window. Register
//! reads/writes that the guest issues as 16- or 32-bit transfers arrive here as
//! individual byte accesses, and the byte-granular field model below reproduces
//! the correct little-endian word/dword semantics.
//!
//! ## What is implemented
//!
//! * **Mixer (NAM)**: Reset (0x00) returning a capabilities word, Master Volume
//!   (0x02), AUX Out (0x04), Mono Volume (0x06), PCM Out Volume (0x18), Record
//!   Select (0x1A), Record Gain (0x1C), General Purpose (0x20), Vendor ID1/ID2
//!   (0x7C/0x7E), and a "codec ready" indication via the Powerdown
//!   Ctrl/Status register (0x26). Volume registers implement the standard
//!   mute bit (bit 15) plus left/right attenuation fields and read back what was
//!   written (masked to the valid bits). Unimplemented mixer registers read as
//!   their power-on default (mostly the muted volume default 0x8000) and accept
//!   writes that read back, so a driver probing the register file sees a
//!   plausible codec.
//!
//! * **Bus Master (NABM)**: three identical DMA channels, each with BDBAR
//!   (Buffer Descriptor list Base Address Register, 32-bit), CIV (Current Index
//!   Value), LVI (Last Valid Index), SR (Status, 16-bit, with RWC bits), PICB
//!   (Position In Current Buffer, 16-bit RO), PIV (Prefetched Index Value, RO),
//!   and CR (Control, 8-bit, including the channel Reset bit). Plus Global
//!   Control (0x2C) and Global Status (0x30).
//!
//! * **Buffer Descriptor List state machine for PCM Out**: [`Self::run_pcm_out`]
//!   reads 8-byte buffer descriptors from guest memory (via the [`Mem`]
//!   abstraction shared with the e1000/virtio models), starting at `CIV`,
//!   "plays" each buffer up to `LVI`, advances `CIV`/`PIV`, and on a descriptor
//!   that carries the Interrupt-On-Completion (IOC) control bit raises the BCIS
//!   status bit. When the run reaches `LVI` the LVBCI (Last Valid Buffer
//!   Completion) bit is raised and the channel halts (DCH set). The
//!   sample bytes themselves are read from guest memory and discarded (written
//!   to a counting sink) — there is no host audio backend.
//!
//! ## What is deferred
//!
//! * Real audio output: PCM samples are read from guest memory and dropped
//!   (only a byte counter is kept). No host sound device, no resampling, no
//!   format conversion (the sample-rate / format mixer registers are stored but
//!   not acted upon).
//! * Interrupt *delivery*: the model computes whether a channel interrupt is
//!   pending (status bits gated by the per-channel interrupt-enable control
//!   bits) and exposes it via [`Self::interrupt_pending`]; wiring it to a PIC /
//!   PCI interrupt line is left to the integration layer, matching e1000.
//! * The PCM In and Mic In capture engines have a full register model but no
//!   data-producing state machine (there is no host capture source); only PCM
//!   Out has the BDL playback walker.
//! * Variable-rate audio (VRA) negotiation beyond storing the bits, S/PDIF, and
//!   the AC'97 2.3 slot-mapping registers.

use crate::devices::bus::IoDevice;

/// Abstraction over guest physical memory used by the bus-master DMA engine.
///
/// Identical in shape to the e1000 / virtio `Mem` trait so the buffer-descriptor
/// walker can be exercised against a flat `Vec<u8>` in unit tests. A real VMM
/// implements this over its guest-memory mapping.
pub trait Mem {
    /// Read bytes starting at guest physical address `gpa` into `buf`.
    /// Returns false on an out-of-bounds access.
    fn read(&self, gpa: u64, buf: &mut [u8]) -> bool;
    /// Write `buf` to guest physical address `gpa`. Returns false on an
    /// out-of-bounds access.
    fn write(&mut self, gpa: u64, buf: &[u8]) -> bool;
}

/// A flat little-endian byte buffer used as guest memory in tests. GPA 0 maps
/// to byte 0 of the buffer.
pub struct VecMem {
    pub bytes: Vec<u8>,
}

impl VecMem {
    pub fn new(size: usize) -> Self {
        VecMem {
            bytes: vec![0u8; size],
        }
    }
}

impl Mem for VecMem {
    fn read(&self, gpa: u64, buf: &mut [u8]) -> bool {
        let start = gpa as usize;
        let end = match start.checked_add(buf.len()) {
            Some(e) => e,
            None => return false,
        };
        if end > self.bytes.len() {
            return false;
        }
        buf.copy_from_slice(&self.bytes[start..end]);
        true
    }

    fn write(&mut self, gpa: u64, buf: &[u8]) -> bool {
        let start = gpa as usize;
        let end = match start.checked_add(buf.len()) {
            Some(e) => e,
            None => return false,
        };
        if end > self.bytes.len() {
            return false;
        }
        self.bytes[start..end].copy_from_slice(buf);
        true
    }
}

// ===========================================================================
// NAM — Native Audio Mixer (codec) register offsets (byte offsets, 16-bit regs)
// ===========================================================================

/// Reset register / capabilities. A write (of any value) triggers a codec reset;
/// a read returns the codec capabilities word.
pub const NAM_RESET: u16 = 0x00;
/// Master Volume (Line Out).
pub const NAM_MASTER_VOLUME: u16 = 0x02;
/// AUX Out / Headphone volume.
pub const NAM_AUX_OUT_VOLUME: u16 = 0x04;
/// Mono Volume.
pub const NAM_MONO_VOLUME: u16 = 0x06;
/// PCM Out Volume.
pub const NAM_PCM_OUT_VOLUME: u16 = 0x18;
/// Record Select (input source mux).
pub const NAM_RECORD_SELECT: u16 = 0x1A;
/// Record Gain.
pub const NAM_RECORD_GAIN: u16 = 0x1C;
/// General Purpose register.
pub const NAM_GENERAL_PURPOSE: u16 = 0x20;
/// Powerdown Control / Status (codec-ready bits live here).
pub const NAM_POWERDOWN: u16 = 0x26;
/// Vendor ID1 (first two characters of the vendor string).
pub const NAM_VENDOR_ID1: u16 = 0x7C;
/// Vendor ID2 (third character + device revision).
pub const NAM_VENDOR_ID2: u16 = 0x7E;

/// Size of the NAM (mixer) I/O window, in bytes.
pub const NAM_SIZE: u16 = 0x100;

/// Mute bit, set in the high bit of a volume register.
pub const VOL_MUTE: u16 = 0x8000;
/// Mask of the writable bits in a stereo volume register: mute + 6-bit left +
/// 6-bit right attenuation (bits 13:8 and 5:0). Bits 14, 7, 6 are reserved/0.
pub const VOL_STEREO_MASK: u16 = VOL_MUTE | 0x3F00 | 0x003F;
/// Mask of the writable bits in a mono volume register: mute + 6-bit right
/// (low) field only.
pub const VOL_MONO_MASK: u16 = VOL_MUTE | 0x003F;

/// Capabilities reported by the Reset register: "Dedicated Mic PCM In channel"
/// (bit 0) plus the bass/treble + headphone capability bits commonly probed.
/// The exact bits are informational; we report headphone (bit 4) + dedicated
/// mic (bit 0).
pub const RESET_CAPABILITIES: u16 = 0x0001 | 0x0010;

/// Powerdown Ctrl/Status: codec-ready composite — ADC, DAC, analog, and
/// reference are all ready (bits 15..12 = REF, ANL, DAC, ADC ready).
pub const POWERDOWN_READY: u16 = 0xF000;

/// Vendor ID. AC'97 vendor IDs are a 3-character vendor string + an 8-bit
/// revision. We advertise a generic "RAX" codec. ID1 holds the first two chars
/// ('R','A'), ID2 holds the third char ('X') in its high byte and a revision in
/// the low byte.
pub const VENDOR_ID1: u16 = ((b'R' as u16) << 8) | (b'A' as u16);
/// Vendor ID2: third char 'X' in the high byte, revision 0x01 in the low byte.
pub const VENDOR_ID2: u16 = ((b'X' as u16) << 8) | 0x01;

// ===========================================================================
// NABM — Native Audio Bus Master register offsets (byte offsets)
// ===========================================================================

/// PCM In channel register block base.
pub const NABM_PCM_IN: u16 = 0x00;
/// PCM Out channel register block base.
pub const NABM_PCM_OUT: u16 = 0x10;
/// Mic In channel register block base.
pub const NABM_MIC_IN: u16 = 0x20;

/// Per-channel register offsets relative to the channel block base.
pub const CH_BDBAR: u16 = 0x00; // 32-bit Buffer Descriptor list Base Address
pub const CH_CIV: u16 = 0x04; // 8-bit Current Index Value (RO)
pub const CH_LVI: u16 = 0x05; // 8-bit Last Valid Index
pub const CH_SR: u16 = 0x06; // 16-bit Status (RWC bits)
pub const CH_PICB: u16 = 0x08; // 16-bit Position In Current Buffer (RO)
pub const CH_PIV: u16 = 0x0A; // 8-bit Prefetched Index Value (RO)
pub const CH_CR: u16 = 0x0B; // 8-bit Control

/// Global Control register (32-bit).
pub const NABM_GLOB_CNT: u16 = 0x2C;
/// Global Status register (32-bit).
pub const NABM_GLOB_STA: u16 = 0x30;

/// Size of the NABM (bus-master) I/O window, in bytes.
pub const NABM_SIZE: u16 = 0x40;

// ---- Status register (SR) bits --------------------------------------------
/// DMA Controller Halted.
pub const SR_DCH: u16 = 1 << 0;
/// Current Equals Last Valid (CIV == LVI).
pub const SR_CELV: u16 = 1 << 1;
/// Last Valid Buffer Completion Interrupt (write-1-to-clear).
pub const SR_LVBCI: u16 = 1 << 2;
/// Buffer Completion Interrupt Status (write-1-to-clear).
pub const SR_BCIS: u16 = 1 << 3;
/// FIFO Error Interrupt (write-1-to-clear).
pub const SR_FIFOE: u16 = 1 << 4;
/// Power-on / reset value of SR: only DMA-halted is set.
pub const SR_RESET: u16 = SR_DCH;
/// Mask of the write-1-to-clear status bits.
pub const SR_RWC_MASK: u16 = SR_LVBCI | SR_BCIS | SR_FIFOE;

// ---- Control register (CR) bits -------------------------------------------
/// Run/Pause Bus Master (1 = run, 0 = pause).
pub const CR_RPBM: u8 = 1 << 0;
/// Reset Registers — self-clearing; resets the channel's registers.
pub const CR_RR: u8 = 1 << 1;
/// Last Valid Buffer Interrupt Enable.
pub const CR_LVBIE: u8 = 1 << 2;
/// FIFO Error Interrupt Enable.
pub const CR_FEIE: u8 = 1 << 3;
/// Interrupt On Completion Interrupt Enable.
pub const CR_IOCE: u8 = 1 << 4;
/// Mask of valid control bits.
pub const CR_MASK: u8 = CR_RPBM | CR_RR | CR_LVBIE | CR_FEIE | CR_IOCE;

// ---- Buffer descriptor control word bits (high 16 bits of dword 1) --------
/// Interrupt On Completion (per-buffer).
pub const BD_IOC: u16 = 1 << 15;
/// Buffer Underrun Policy / "Buffer Last" (stop after this buffer).
pub const BD_BUP: u16 = 1 << 14;

/// Number of entries in a Buffer Descriptor List (the index registers are
/// 5-bit, so the list is 32 entries and indices wrap modulo 32).
pub const BDL_ENTRIES: u8 = 32;

/// Size of one buffer descriptor in guest memory, in bytes.
pub const BD_SIZE: u64 = 8;

// ===========================================================================
// Bus-master channel
// ===========================================================================

/// State of a single AC'97 bus-master DMA channel.
#[derive(Clone, Copy, Debug)]
pub struct BmChannel {
    /// Buffer Descriptor list Base Address (32-bit; low 3 bits forced to 0).
    pub bdbar: u32,
    /// Current Index Value (which BDL entry is being processed).
    pub civ: u8,
    /// Last Valid Index (the last entry the driver has filled).
    pub lvi: u8,
    /// Status register.
    pub sr: u16,
    /// Position In Current Buffer (samples remaining in the current buffer).
    pub picb: u16,
    /// Prefetched Index Value.
    pub piv: u8,
    /// Control register.
    pub cr: u8,
}

impl Default for BmChannel {
    fn default() -> Self {
        BmChannel {
            bdbar: 0,
            civ: 0,
            lvi: 0,
            sr: SR_RESET,
            picb: 0,
            piv: 0,
            cr: 0,
        }
    }
}

impl BmChannel {
    /// Reset the channel registers to their power-on state (driven by the CR
    /// Reset-Registers bit). BDBAR is preserved across an RR per the ICH spec;
    /// the index/status/position registers are cleared. SR returns to DCH-set.
    pub fn reset_registers(&mut self) {
        self.civ = 0;
        self.lvi = 0;
        self.picb = 0;
        self.piv = 0;
        self.sr = SR_RESET;
        // The Run/Pause bit is cleared by reset; other CR bits are cleared too.
        self.cr = 0;
    }

    /// Whether the channel currently has a pending interrupt: any status bit
    /// whose enable is set in CR.
    pub fn interrupt_pending(&self) -> bool {
        let mut pending = false;
        if self.cr & CR_IOCE != 0 && self.sr & SR_BCIS != 0 {
            pending = true;
        }
        if self.cr & CR_LVBIE != 0 && self.sr & SR_LVBCI != 0 {
            pending = true;
        }
        if self.cr & CR_FEIE != 0 && self.sr & SR_FIFOE != 0 {
            pending = true;
        }
        pending
    }
}

/// Which bus-master channel a NABM offset refers to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChannelId {
    PcmIn,
    PcmOut,
    MicIn,
}

// ===========================================================================
// The device
// ===========================================================================

/// Intel AC'97 audio controller.
pub struct Ac97 {
    // -- NAM / mixer (codec) registers, indexed by byte offset / 2 --
    /// The 128 16-bit codec registers (256-byte window).
    mixer: [u16; 128],

    // -- NABM / bus-master channels --
    pcm_in: BmChannel,
    pcm_out: BmChannel,
    mic_in: BmChannel,

    /// Global Control register.
    glob_cnt: u32,
    /// Global Status register.
    glob_sta: u32,

    /// Total number of PCM-out sample bytes "played" (read from guest memory and
    /// discarded). Acts as the audio sink for testing.
    sink_bytes: u64,
}

impl Default for Ac97 {
    fn default() -> Self {
        Self::new()
    }
}

impl Ac97 {
    /// Global Status: primary codec ready (bit 8). Set at power-on so drivers
    /// see a codec.
    const GLOB_STA_PCR: u32 = 1 << 8;

    /// Construct an AC'97 controller in its power-on reset state.
    pub fn new() -> Self {
        let mut mixer = [0u16; 128];
        // Reset register reports capabilities.
        mixer[(NAM_RESET / 2) as usize] = RESET_CAPABILITIES;
        // Volume registers power up muted at full attenuation (the AC'97
        // default of 0x8000 — mute set).
        mixer[(NAM_MASTER_VOLUME / 2) as usize] = VOL_MUTE;
        mixer[(NAM_AUX_OUT_VOLUME / 2) as usize] = VOL_MUTE;
        mixer[(NAM_MONO_VOLUME / 2) as usize] = VOL_MUTE;
        mixer[(NAM_PCM_OUT_VOLUME / 2) as usize] = VOL_MUTE;
        // Codec ready.
        mixer[(NAM_POWERDOWN / 2) as usize] = POWERDOWN_READY;
        // Vendor identification.
        mixer[(NAM_VENDOR_ID1 / 2) as usize] = VENDOR_ID1;
        mixer[(NAM_VENDOR_ID2 / 2) as usize] = VENDOR_ID2;

        Ac97 {
            mixer,
            pcm_in: BmChannel::default(),
            pcm_out: BmChannel::default(),
            mic_in: BmChannel::default(),
            glob_cnt: 0,
            glob_sta: Self::GLOB_STA_PCR,
            sink_bytes: 0,
        }
    }

    // ---- Mixer (NAM) ------------------------------------------------------

    /// Read a 16-bit mixer register at byte `offset` (must be even / in range).
    pub fn mixer_read(&self, offset: u16) -> u16 {
        let idx = (offset / 2) as usize;
        if idx < self.mixer.len() {
            self.mixer[idx]
        } else {
            0
        }
    }

    /// Write a 16-bit mixer register at byte `offset`, applying per-register
    /// semantics (read-only fields, mute/attenuation masking, reset trigger).
    pub fn mixer_write(&mut self, offset: u16, value: u16) {
        let idx = (offset / 2) as usize;
        if idx >= self.mixer.len() {
            return;
        }
        match offset {
            NAM_RESET => {
                // Any write triggers a codec reset: capabilities are restored
                // and the mixer returns to its power-on defaults.
                self.reset_codec();
            }
            NAM_MASTER_VOLUME | NAM_AUX_OUT_VOLUME | NAM_PCM_OUT_VOLUME => {
                self.mixer[idx] = value & VOL_STEREO_MASK;
            }
            NAM_MONO_VOLUME => {
                self.mixer[idx] = value & VOL_MONO_MASK;
            }
            NAM_VENDOR_ID1 | NAM_VENDOR_ID2 => {
                // Vendor IDs are read-only.
            }
            _ => {
                // Generic read/write register (record select/gain, general
                // purpose, and unimplemented registers).
                self.mixer[idx] = value;
            }
        }
    }

    /// Restore the mixer to its power-on defaults (codec reset).
    pub fn reset_codec(&mut self) {
        self.mixer = [0u16; 128];
        self.mixer[(NAM_RESET / 2) as usize] = RESET_CAPABILITIES;
        self.mixer[(NAM_MASTER_VOLUME / 2) as usize] = VOL_MUTE;
        self.mixer[(NAM_AUX_OUT_VOLUME / 2) as usize] = VOL_MUTE;
        self.mixer[(NAM_MONO_VOLUME / 2) as usize] = VOL_MUTE;
        self.mixer[(NAM_PCM_OUT_VOLUME / 2) as usize] = VOL_MUTE;
        self.mixer[(NAM_POWERDOWN / 2) as usize] = POWERDOWN_READY;
        self.mixer[(NAM_VENDOR_ID1 / 2) as usize] = VENDOR_ID1;
        self.mixer[(NAM_VENDOR_ID2 / 2) as usize] = VENDOR_ID2;
    }

    /// Whether the codec reports itself ready (used by drivers as a probe).
    pub fn codec_ready(&self) -> bool {
        self.mixer_read(NAM_POWERDOWN) & POWERDOWN_READY == POWERDOWN_READY
    }

    /// Whether a stereo volume register is muted.
    pub fn is_muted(&self, offset: u16) -> bool {
        self.mixer_read(offset) & VOL_MUTE != 0
    }

    // ---- Channel selection ------------------------------------------------

    fn channel(&self, id: ChannelId) -> &BmChannel {
        match id {
            ChannelId::PcmIn => &self.pcm_in,
            ChannelId::PcmOut => &self.pcm_out,
            ChannelId::MicIn => &self.mic_in,
        }
    }

    fn channel_mut(&mut self, id: ChannelId) -> &mut BmChannel {
        match id {
            ChannelId::PcmIn => &mut self.pcm_in,
            ChannelId::PcmOut => &mut self.pcm_out,
            ChannelId::MicIn => &mut self.mic_in,
        }
    }

    /// Public accessor for a channel's register state (for tests / integration).
    pub fn channel_state(&self, id: ChannelId) -> BmChannel {
        *self.channel(id)
    }

    /// Decode a NABM byte offset into (channel, register-offset-within-channel),
    /// or `None` if it lands in the global register area.
    fn decode_channel(offset: u16) -> Option<(ChannelId, u16)> {
        match offset {
            NABM_PCM_IN..=0x0F => Some((ChannelId::PcmIn, offset - NABM_PCM_IN)),
            NABM_PCM_OUT..=0x1F => Some((ChannelId::PcmOut, offset - NABM_PCM_OUT)),
            NABM_MIC_IN..=0x2B => Some((ChannelId::MicIn, offset - NABM_MIC_IN)),
            _ => None,
        }
    }

    // ---- Bus master (NABM) register access --------------------------------

    /// Read a single byte from a NABM register at byte `offset`.
    pub fn nabm_read_u8(&self, offset: u16) -> u8 {
        if let Some((id, reg)) = Self::decode_channel(offset) {
            let ch = self.channel(id);
            match reg {
                CH_BDBAR..=0x03 => (ch.bdbar >> (8 * (reg - CH_BDBAR))) as u8,
                CH_CIV => ch.civ,
                CH_LVI => ch.lvi,
                CH_SR => ch.sr as u8,
                0x07 => (ch.sr >> 8) as u8,
                CH_PICB => ch.picb as u8,
                0x09 => (ch.picb >> 8) as u8,
                CH_PIV => ch.piv,
                CH_CR => ch.cr,
                _ => 0,
            }
        } else {
            match offset {
                NABM_GLOB_CNT..=0x2F => (self.glob_cnt >> (8 * (offset - NABM_GLOB_CNT))) as u8,
                NABM_GLOB_STA..=0x33 => (self.glob_sta >> (8 * (offset - NABM_GLOB_STA))) as u8,
                _ => 0,
            }
        }
    }

    /// Write a single byte to a NABM register at byte `offset`.
    pub fn nabm_write_u8(&mut self, offset: u16, value: u8) {
        if let Some((id, reg)) = Self::decode_channel(offset) {
            match reg {
                CH_BDBAR..=0x03 => {
                    let shift = 8 * (reg - CH_BDBAR);
                    let mask = !(0xFFu32 << shift);
                    let mut v = (self.channel(id).bdbar & mask) | ((value as u32) << shift);
                    // BDL base must be 8-byte aligned: low 3 bits read as 0.
                    v &= !0x7u32;
                    self.channel_mut(id).bdbar = v;
                }
                CH_CIV => { /* Current Index Value is read-only. */ }
                CH_LVI => {
                    let ch = self.channel_mut(id);
                    ch.lvi = value;
                    // Programming a new LVI while running clears the halted/CELV
                    // condition so the engine can resume.
                    if ch.cr & CR_RPBM != 0 {
                        ch.sr &= !(SR_DCH | SR_CELV);
                    }
                }
                CH_SR => {
                    // Low byte: write-1-to-clear the RWC bits.
                    let clear = (value as u16) & SR_RWC_MASK;
                    self.channel_mut(id).sr &= !clear;
                }
                0x07 => { /* SR high byte: reserved. */ }
                CH_PICB | 0x09 => { /* PICB is read-only. */ }
                CH_PIV => { /* PIV is read-only. */ }
                CH_CR => self.write_cr(id, value),
                _ => {}
            }
        } else {
            match offset {
                NABM_GLOB_CNT..=0x2F => {
                    let shift = 8 * (offset - NABM_GLOB_CNT);
                    let mask = !(0xFFu32 << shift);
                    self.glob_cnt = (self.glob_cnt & mask) | ((value as u32) << shift);
                }
                NABM_GLOB_STA..=0x33 => {
                    // Bits in GLOB_STA are mostly RO / RWC; we treat the whole
                    // register as RO here except we let the driver clear the
                    // RWC interrupt bits via write-1-to-clear semantics.
                    let shift = 8 * (offset - NABM_GLOB_STA);
                    let cleared = (value as u32) << shift;
                    // Only the lower interrupt/RWC bits are clearable; PCR (bit
                    // 8) and similar capability bits are sticky.
                    self.glob_sta &= !(cleared & 0x0000_00FF);
                }
                _ => {}
            }
        }
    }

    /// Apply a write to a channel's Control register, handling the Reset-
    /// Registers bit (self-clearing) and the run/pause + interrupt-enable bits.
    fn write_cr(&mut self, id: ChannelId, value: u8) {
        if value & CR_RR != 0 {
            // Reset Registers: only honoured while the channel is paused
            // (RPBM == 0), per the spec. It resets the channel and is
            // self-clearing (reads back as 0).
            let paused = self.channel(id).cr & CR_RPBM == 0;
            if paused {
                self.channel_mut(id).reset_registers();
                return;
            }
        }
        let ch = self.channel_mut(id);
        let new = value & CR_MASK & !CR_RR; // RR never stays set
        let was_running = ch.cr & CR_RPBM != 0;
        ch.cr = new;
        if new & CR_RPBM != 0 && !was_running {
            // Transition to running: clear the halted bit so the engine can
            // start processing on the next run.
            ch.sr &= !SR_DCH;
        }
        if new & CR_RPBM == 0 {
            // Paused: mark the controller halted.
            ch.sr |= SR_DCH;
        }
    }

    /// 64-bit guest base address of a channel's buffer descriptor list.
    pub fn bdl_base(&self, id: ChannelId) -> u64 {
        u64::from(self.channel(id).bdbar)
    }

    /// Total bytes "played" through the discard sink.
    pub fn sink_bytes(&self) -> u64 {
        self.sink_bytes
    }

    /// True when any bus-master channel has a pending (enabled) interrupt.
    pub fn interrupt_pending(&self) -> bool {
        self.pcm_in.interrupt_pending()
            || self.pcm_out.interrupt_pending()
            || self.mic_in.interrupt_pending()
    }

    // ---- PCM Out buffer-descriptor-list state machine ---------------------

    /// Run the PCM Out DMA engine: walk the buffer descriptor list from `CIV`
    /// up to and including `LVI`, reading each buffer's samples from guest
    /// memory (and discarding them to the sink), advancing `CIV`/`PIV`, and
    /// raising the appropriate status bits.
    ///
    /// The engine only runs when the channel's Run bit (`CR.RPBM`) is set. Each
    /// buffer descriptor is 8 bytes: dword 0 is the buffer's guest physical
    /// address, dword 1 holds the sample count (number of 16-bit samples) in
    /// its low 16 bits and the control flags (IOC/BUP) in its high 16 bits.
    ///
    /// Returns the number of buffers processed in this call.
    pub fn run_pcm_out<M: Mem>(&mut self, mem: &mut M) -> usize {
        if self.pcm_out.cr & CR_RPBM == 0 {
            return 0;
        }
        let base = u64::from(self.pcm_out.bdbar);
        let mut processed = 0usize;
        // Bound the walk to one full traversal of the 32-entry list to prevent
        // a runaway loop on malformed descriptors.
        let mut guard = BDL_ENTRIES as usize + 1;

        loop {
            if guard == 0 {
                break;
            }
            guard -= 1;

            // Halt once we've moved past the last valid index.
            if self.pcm_out.sr & SR_DCH != 0 {
                break;
            }

            let civ = self.pcm_out.civ;
            let bd_gpa = base + u64::from(civ) * BD_SIZE;
            let mut raw = [0u8; BD_SIZE as usize];
            if !mem.read(bd_gpa, &mut raw) {
                // DMA fault: flag a FIFO error and halt.
                self.pcm_out.sr |= SR_FIFOE | SR_DCH;
                break;
            }

            let buf_addr = u32::from_le_bytes(raw[0..4].try_into().unwrap());
            let ctrl_len = u32::from_le_bytes(raw[4..8].try_into().unwrap());
            let samples = (ctrl_len & 0xFFFF) as u16; // 16-bit sample count
            let control = (ctrl_len >> 16) as u16;

            // Number of bytes in this buffer (each sample is 16 bits).
            let nbytes = u64::from(samples) * 2;
            self.pcm_out.picb = samples;

            // "Play" the buffer: read it from guest memory and discard.
            if nbytes > 0 {
                let mut sink = vec![0u8; nbytes as usize];
                if mem.read(u64::from(buf_addr), &mut sink) {
                    self.sink_bytes += nbytes;
                } else {
                    self.pcm_out.sr |= SR_FIFOE | SR_DCH;
                    break;
                }
            }
            // Buffer fully consumed.
            self.pcm_out.picb = 0;
            processed += 1;

            let at_lvi = civ == self.pcm_out.lvi;

            // Raise the per-buffer interrupt-on-completion status if requested.
            if control & BD_IOC != 0 {
                self.pcm_out.sr |= SR_BCIS;
            }

            if at_lvi {
                // Reached the last valid buffer: raise LVBCI, mark CIV==LVI, and
                // halt the engine.
                self.pcm_out.sr |= SR_LVBCI | SR_CELV | SR_DCH;
                self.pcm_out.piv = civ;
                break;
            }

            // Advance to the next descriptor (indices wrap modulo 32).
            let next = (civ + 1) % BDL_ENTRIES;
            self.pcm_out.civ = next;
            self.pcm_out.piv = next;

            // Honour a "buffer last" (BUP) descriptor by stopping after it even
            // before LVI (rare; used to drain).
            if control & BD_BUP != 0 {
                self.pcm_out.sr |= SR_DCH;
                break;
            }
        }

        processed
    }

    // ---- PIO port decode --------------------------------------------------

    /// Construct an [`Ac97Pio`] wrapper that decodes the contiguous I/O port
    /// range starting at `nam_base` (NAM, 256 bytes) immediately followed by
    /// `nabm_base` (NABM, 64 bytes). When the two BARs are not contiguous, use
    /// two separate [`Ac97PioWindow`]s sharing the device.
    pub fn pio(nam_base: u16, nabm_base: u16) -> Ac97Pio {
        Ac97Pio {
            dev: Ac97::new(),
            nam_base,
            nabm_base,
        }
    }
}

// ===========================================================================
// PIO adapter
// ===========================================================================

/// An [`IoDevice`] wrapper that owns an [`Ac97`] and decodes accesses to its
/// two I/O windows (NAM and NABM) by base port.
///
/// Codec (NAM) registers are 16-bit; the bus dispatches byte-by-byte, so word
/// reads/writes arrive as two consecutive single-byte accesses. To present
/// coherent 16-bit semantics (the mute/reset/RO logic must see the whole word),
/// byte writes to a register are buffered: the low byte is latched and the
/// 16-bit register update is applied when the high byte arrives. Reads return
/// the appropriate byte of the current register value, which is consistent for
/// both byte and word reads.
pub struct Ac97Pio {
    dev: Ac97,
    nam_base: u16,
    nabm_base: u16,
}

impl Ac97Pio {
    /// Borrow the underlying device (e.g. to drive [`Ac97::run_pcm_out`]).
    pub fn device(&mut self) -> &mut Ac97 {
        &mut self.dev
    }

    /// Borrow the underlying device immutably.
    pub fn device_ref(&self) -> &Ac97 {
        &self.dev
    }

    fn nam_offset(&self, port: u16) -> Option<u16> {
        if port >= self.nam_base && port < self.nam_base.saturating_add(NAM_SIZE) {
            Some(port - self.nam_base)
        } else {
            None
        }
    }

    fn nabm_offset(&self, port: u16) -> Option<u16> {
        if port >= self.nabm_base && port < self.nabm_base.saturating_add(NABM_SIZE) {
            Some(port - self.nabm_base)
        } else {
            None
        }
    }
}

impl IoDevice for Ac97Pio {
    fn read(&mut self, port: u16) -> u8 {
        if let Some(off) = self.nam_offset(port) {
            // Return the appropriate byte of the 16-bit register.
            let reg = off & !1;
            let word = self.dev.mixer_read(reg);
            if off & 1 == 0 {
                word as u8
            } else {
                (word >> 8) as u8
            }
        } else if let Some(off) = self.nabm_offset(port) {
            self.dev.nabm_read_u8(off)
        } else {
            0xff
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Some(off) = self.nam_offset(port) {
            // Apply the register write per-byte. Reads/writes of mixer registers
            // are word-oriented; we update the register's byte and re-apply the
            // word semantics so the mute/RO/reset logic stays correct.
            let reg = off & !1;
            let cur = self.dev.mixer_read(reg);
            let new = if off & 1 == 0 {
                (cur & 0xFF00) | u16::from(value)
            } else {
                (cur & 0x00FF) | (u16::from(value) << 8)
            };
            self.dev.mixer_write(reg, new);
        } else if let Some(off) = self.nabm_offset(port) {
            self.dev.nabm_write_u8(off, value);
        }
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn write_bd(
        mem: &mut VecMem,
        bdl_base: u64,
        index: u8,
        buf_addr: u32,
        samples: u16,
        ctrl: u16,
    ) {
        let gpa = bdl_base + u64::from(index) * BD_SIZE;
        let mut raw = [0u8; 8];
        raw[0..4].copy_from_slice(&buf_addr.to_le_bytes());
        let ctrl_len = (u32::from(ctrl) << 16) | u32::from(samples);
        raw[4..8].copy_from_slice(&ctrl_len.to_le_bytes());
        assert!(mem.write(gpa, &raw));
    }

    // ---- Mixer / NAM ------------------------------------------------------

    #[test]
    fn reset_register_reports_capabilities() {
        let dev = Ac97::new();
        assert_eq!(dev.mixer_read(NAM_RESET), RESET_CAPABILITIES);
        assert_ne!(RESET_CAPABILITIES, 0);
    }

    #[test]
    fn vendor_id_registers() {
        let dev = Ac97::new();
        assert_eq!(dev.mixer_read(NAM_VENDOR_ID1), VENDOR_ID1);
        assert_eq!(dev.mixer_read(NAM_VENDOR_ID2), VENDOR_ID2);
        // Vendor IDs are read-only.
        let mut dev = dev;
        dev.mixer_write(NAM_VENDOR_ID1, 0x1234);
        dev.mixer_write(NAM_VENDOR_ID2, 0x5678);
        assert_eq!(dev.mixer_read(NAM_VENDOR_ID1), VENDOR_ID1);
        assert_eq!(dev.mixer_read(NAM_VENDOR_ID2), VENDOR_ID2);
    }

    #[test]
    fn codec_ready_on_reset() {
        let dev = Ac97::new();
        assert!(dev.codec_ready());
        assert_eq!(dev.mixer_read(NAM_POWERDOWN), POWERDOWN_READY);
    }

    #[test]
    fn master_volume_read_write_and_mute_bit() {
        let mut dev = Ac97::new();
        // Powers up muted.
        assert!(dev.is_muted(NAM_MASTER_VOLUME));
        // Program a mid attenuation, unmuted.
        dev.mixer_write(NAM_MASTER_VOLUME, 0x0808);
        assert_eq!(dev.mixer_read(NAM_MASTER_VOLUME), 0x0808);
        assert!(!dev.is_muted(NAM_MASTER_VOLUME));
        // Set the mute bit.
        dev.mixer_write(NAM_MASTER_VOLUME, VOL_MUTE | 0x0808);
        assert!(dev.is_muted(NAM_MASTER_VOLUME));
        assert_eq!(dev.mixer_read(NAM_MASTER_VOLUME), VOL_MUTE | 0x0808);
    }

    #[test]
    fn volume_reserved_bits_masked_off() {
        let mut dev = Ac97::new();
        // Bits 14, 7, 6 are reserved and must read back as 0.
        dev.mixer_write(NAM_PCM_OUT_VOLUME, 0xFFFF);
        assert_eq!(dev.mixer_read(NAM_PCM_OUT_VOLUME), VOL_STEREO_MASK);
        assert_eq!(dev.mixer_read(NAM_PCM_OUT_VOLUME) & 0x4000, 0);
    }

    #[test]
    fn mono_volume_masks_to_mono_field() {
        let mut dev = Ac97::new();
        dev.mixer_write(NAM_MONO_VOLUME, 0xFFFF);
        assert_eq!(dev.mixer_read(NAM_MONO_VOLUME), VOL_MONO_MASK);
    }

    #[test]
    fn record_select_and_gain_general_purpose_rw() {
        let mut dev = Ac97::new();
        dev.mixer_write(NAM_RECORD_SELECT, 0x0303);
        assert_eq!(dev.mixer_read(NAM_RECORD_SELECT), 0x0303);
        dev.mixer_write(NAM_RECORD_GAIN, 0x0A0A);
        assert_eq!(dev.mixer_read(NAM_RECORD_GAIN), 0x0A0A);
        dev.mixer_write(NAM_GENERAL_PURPOSE, 0x9000);
        assert_eq!(dev.mixer_read(NAM_GENERAL_PURPOSE), 0x9000);
    }

    #[test]
    fn writing_reset_register_restores_defaults() {
        let mut dev = Ac97::new();
        dev.mixer_write(NAM_MASTER_VOLUME, 0x0101);
        dev.mixer_write(NAM_GENERAL_PURPOSE, 0x1234);
        assert_eq!(dev.mixer_read(NAM_MASTER_VOLUME), 0x0101);
        // Any write to the reset register triggers a codec reset.
        dev.mixer_write(NAM_RESET, 0x0000);
        assert_eq!(dev.mixer_read(NAM_MASTER_VOLUME), VOL_MUTE);
        assert_eq!(dev.mixer_read(NAM_GENERAL_PURPOSE), 0);
        assert_eq!(dev.mixer_read(NAM_RESET), RESET_CAPABILITIES);
        assert!(dev.codec_ready());
    }

    // ---- NABM register access ---------------------------------------------

    #[test]
    fn nabm_channel_decode() {
        assert_eq!(Ac97::decode_channel(0x00), Some((ChannelId::PcmIn, 0x00)));
        assert_eq!(Ac97::decode_channel(0x10), Some((ChannelId::PcmOut, 0x00)));
        assert_eq!(Ac97::decode_channel(0x1B), Some((ChannelId::PcmOut, 0x0B)));
        assert_eq!(Ac97::decode_channel(0x20), Some((ChannelId::MicIn, 0x00)));
        assert_eq!(Ac97::decode_channel(NABM_GLOB_CNT), None);
        assert_eq!(Ac97::decode_channel(NABM_GLOB_STA), None);
    }

    #[test]
    fn bdbar_programming_byte_wise_and_alignment() {
        let mut dev = Ac97::new();
        let off = NABM_PCM_OUT + CH_BDBAR;
        // Program a 32-bit, byte-by-byte (low byte unaligned to test masking).
        dev.nabm_write_u8(off, 0xFF);
        dev.nabm_write_u8(off + 1, 0xBE);
        dev.nabm_write_u8(off + 2, 0xAD);
        dev.nabm_write_u8(off + 3, 0xDE);
        // Low 3 bits are forced to zero (8-byte alignment).
        assert_eq!(dev.channel_state(ChannelId::PcmOut).bdbar, 0xDEAD_BEF8);
        assert_eq!(dev.bdl_base(ChannelId::PcmOut), 0xDEAD_BEF8);
        // Read back byte-wise.
        assert_eq!(dev.nabm_read_u8(off), 0xF8);
        assert_eq!(dev.nabm_read_u8(off + 1), 0xBE);
        assert_eq!(dev.nabm_read_u8(off + 2), 0xAD);
        assert_eq!(dev.nabm_read_u8(off + 3), 0xDE);
    }

    #[test]
    fn lvi_programming_and_civ_readonly() {
        let mut dev = Ac97::new();
        let lvi_off = NABM_PCM_OUT + CH_LVI;
        dev.nabm_write_u8(lvi_off, 0x05);
        assert_eq!(dev.channel_state(ChannelId::PcmOut).lvi, 0x05);
        assert_eq!(dev.nabm_read_u8(lvi_off), 0x05);
        // CIV is read-only: writes are ignored.
        let civ_off = NABM_PCM_OUT + CH_CIV;
        dev.nabm_write_u8(civ_off, 0x07);
        assert_eq!(dev.channel_state(ChannelId::PcmOut).civ, 0x00);
    }

    #[test]
    fn control_register_bits_and_run() {
        let mut dev = Ac97::new();
        let cr_off = NABM_PCM_OUT + CH_CR;
        dev.nabm_write_u8(cr_off, CR_RPBM | CR_IOCE | CR_LVBIE);
        let ch = dev.channel_state(ChannelId::PcmOut);
        assert_eq!(ch.cr, CR_RPBM | CR_IOCE | CR_LVBIE);
        // Running clears the DMA-halted bit.
        assert_eq!(ch.sr & SR_DCH, 0);
        // Pausing sets it again.
        dev.nabm_write_u8(cr_off, 0);
        assert_ne!(dev.channel_state(ChannelId::PcmOut).sr & SR_DCH, 0);
    }

    #[test]
    fn control_reset_bit_self_clears_and_resets_channel() {
        let mut dev = Ac97::new();
        let cr_off = NABM_PCM_OUT + CH_CR;
        let lvi_off = NABM_PCM_OUT + CH_LVI;
        let bdbar_off = NABM_PCM_OUT + CH_BDBAR;
        // Set up some state (channel paused).
        dev.nabm_write_u8(lvi_off, 0x0A);
        dev.nabm_write_u8(bdbar_off, 0x00);
        dev.nabm_write_u8(bdbar_off + 1, 0x10);
        // Inject a status bit to verify it gets cleared.
        dev.channel_mut(ChannelId::PcmOut).sr |= SR_BCIS;
        // Reset Registers.
        dev.nabm_write_u8(cr_off, CR_RR);
        let ch = dev.channel_state(ChannelId::PcmOut);
        // RR is self-clearing.
        assert_eq!(ch.cr & CR_RR, 0);
        assert_eq!(ch.lvi, 0);
        assert_eq!(ch.civ, 0);
        assert_eq!(ch.sr, SR_RESET);
        // BDBAR is preserved across a register reset.
        assert_eq!(ch.bdbar, 0x1000);
    }

    #[test]
    fn status_rwc_bits_write_one_to_clear() {
        let mut dev = Ac97::new();
        let sr_off = NABM_PCM_OUT + CH_SR;
        // Seed all the RWC status bits.
        dev.channel_mut(ChannelId::PcmOut).sr |= SR_LVBCI | SR_BCIS | SR_FIFOE;
        // Read back the low byte.
        let lo = dev.nabm_read_u8(sr_off);
        assert_ne!(lo & (SR_BCIS as u8), 0);
        // Write-1-to-clear just BCIS.
        dev.nabm_write_u8(sr_off, SR_BCIS as u8);
        assert_eq!(dev.channel_state(ChannelId::PcmOut).sr & SR_BCIS, 0);
        // LVBCI and FIFOE remain.
        assert_ne!(dev.channel_state(ChannelId::PcmOut).sr & SR_LVBCI, 0);
        assert_ne!(dev.channel_state(ChannelId::PcmOut).sr & SR_FIFOE, 0);
        // Clear the rest.
        dev.nabm_write_u8(sr_off, (SR_LVBCI | SR_FIFOE) as u8);
        assert_eq!(dev.channel_state(ChannelId::PcmOut).sr & SR_RWC_MASK, 0);
        // DCH (not RWC) is unaffected by a 1-write.
        let dch_before = dev.channel_state(ChannelId::PcmOut).sr & SR_DCH;
        dev.nabm_write_u8(sr_off, SR_DCH as u8);
        assert_eq!(dev.channel_state(ChannelId::PcmOut).sr & SR_DCH, dch_before);
    }

    #[test]
    fn global_control_and_status() {
        let mut dev = Ac97::new();
        // Primary codec ready is set at power-on.
        assert_ne!(dev.nabm_read_u8(NABM_GLOB_STA + 1) & 0x01, 0); // bit 8
        // Program global control byte-wise (e.g. enable cold reset de-assert).
        dev.nabm_write_u8(NABM_GLOB_CNT, 0x02);
        assert_eq!(dev.nabm_read_u8(NABM_GLOB_CNT), 0x02);
        assert_eq!(dev.glob_cnt & 0xFF, 0x02);
    }

    // ---- PCM Out BDL state machine -----------------------------------------

    #[test]
    fn pcm_out_engine_idle_when_not_running() {
        let mut dev = Ac97::new();
        let mut mem = VecMem::new(0x1000);
        assert_eq!(dev.run_pcm_out(&mut mem), 0);
        assert_eq!(dev.sink_bytes(), 0);
    }

    #[test]
    fn pcm_out_plays_buffers_and_advances_civ() {
        let mut dev = Ac97::new();
        let mut mem = VecMem::new(0x4000);
        let bdl = 0x1000u64;
        // Two buffers of 8 16-bit samples each; second carries IOC.
        write_bd(&mut mem, bdl, 0, 0x2000, 8, 0);
        write_bd(&mut mem, bdl, 1, 0x2100, 8, BD_IOC);

        // Program BDBAR, LVI=1, and run.
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR, (bdl & 0xFF) as u8);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR + 1, ((bdl >> 8) & 0xFF) as u8);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_LVI, 1);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_CR, CR_RPBM | CR_IOCE | CR_LVBIE);

        let processed = dev.run_pcm_out(&mut mem);
        assert_eq!(processed, 2);
        // 2 buffers * 8 samples * 2 bytes = 32 bytes through the sink.
        assert_eq!(dev.sink_bytes(), 32);

        let ch = dev.channel_state(ChannelId::PcmOut);
        // Reached LVI: BCIS (from buffer 1's IOC), LVBCI, CELV and DCH set.
        assert_ne!(ch.sr & SR_BCIS, 0);
        assert_ne!(ch.sr & SR_LVBCI, 0);
        assert_ne!(ch.sr & SR_CELV, 0);
        assert_ne!(ch.sr & SR_DCH, 0);
        assert_eq!(ch.civ, 1);
        // With IOCE and LVBIE enabled, an interrupt is pending.
        assert!(dev.interrupt_pending());
    }

    #[test]
    fn pcm_out_dma_fault_sets_fifo_error() {
        let mut dev = Ac97::new();
        let mut mem = VecMem::new(0x100); // too small to hold the BDL
        // Point BDBAR past the end of guest memory.
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR, 0x00);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR + 1, 0x80); // 0x8000
        dev.nabm_write_u8(NABM_PCM_OUT + CH_LVI, 0);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_CR, CR_RPBM | CR_FEIE);
        let processed = dev.run_pcm_out(&mut mem);
        assert_eq!(processed, 0);
        let ch = dev.channel_state(ChannelId::PcmOut);
        assert_ne!(ch.sr & SR_FIFOE, 0);
        assert_ne!(ch.sr & SR_DCH, 0);
        assert!(dev.interrupt_pending());
    }

    #[test]
    fn lvi_write_while_running_clears_halt() {
        let mut dev = Ac97::new();
        // Start running, then have the engine halt at LVI.
        let mut mem = VecMem::new(0x4000);
        let bdl = 0x800u64;
        write_bd(&mut mem, bdl, 0, 0x2000, 4, BD_IOC);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR, (bdl & 0xFF) as u8);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_BDBAR + 1, ((bdl >> 8) & 0xFF) as u8);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_LVI, 0);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_CR, CR_RPBM);
        dev.run_pcm_out(&mut mem);
        assert_ne!(dev.channel_state(ChannelId::PcmOut).sr & SR_DCH, 0);
        // Driver appends a new buffer and bumps LVI while still running.
        write_bd(&mut mem, bdl, 1, 0x2100, 4, BD_IOC);
        dev.nabm_write_u8(NABM_PCM_OUT + CH_LVI, 1);
        // DCH / CELV cleared so the engine can resume.
        let ch = dev.channel_state(ChannelId::PcmOut);
        assert_eq!(ch.sr & SR_DCH, 0);
        assert_eq!(ch.sr & SR_CELV, 0);
    }

    // ---- PIO adapter -------------------------------------------------------

    #[test]
    fn pio_word_access_mixer() {
        let mut pio = Ac97::pio(0x1000, 0x1400);
        // Word-read the vendor ID register byte by byte (LE).
        let lo = pio.read(0x1000 + NAM_VENDOR_ID1);
        let hi = pio.read(0x1000 + NAM_VENDOR_ID1 + 1);
        assert_eq!(u16::from(lo) | (u16::from(hi) << 8), VENDOR_ID1);

        // Word-write master volume: low byte then high byte.
        pio.write(0x1000 + NAM_MASTER_VOLUME, 0x08);
        pio.write(0x1000 + NAM_MASTER_VOLUME + 1, 0x80 | 0x08); // mute + atten
        assert_eq!(
            pio.device_ref().mixer_read(NAM_MASTER_VOLUME),
            VOL_MUTE | 0x0808
        );
    }

    #[test]
    fn pio_nabm_access() {
        let mut pio = Ac97::pio(0x1000, 0x1400);
        // Program LVI via the bus-master window.
        pio.write(0x1400 + NABM_PCM_OUT + CH_LVI, 0x03);
        assert_eq!(pio.read(0x1400 + NABM_PCM_OUT + CH_LVI), 0x03);
        assert_eq!(pio.device_ref().channel_state(ChannelId::PcmOut).lvi, 0x03);
    }

    #[test]
    fn pio_out_of_range_reads_ff() {
        let mut pio = Ac97::pio(0x1000, 0x1400);
        assert_eq!(pio.read(0x2000), 0xff);
    }
}
