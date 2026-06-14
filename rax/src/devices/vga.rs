//! Standard VGA (IBM VGA / compatible) device emulation.
//!
//! This implements the register-level state of a standard VGA adapter exposed
//! through programmed I/O (PIO), plus a backing framebuffer covering the legacy
//! video memory aperture 0xA0000-0xBFFFF (128 KiB). It does not perform actual
//! rasterization; instead it maintains enough state (indexed register files, the
//! DAC palette, and raw video memory) for a future MMIO hookup and/or renderer
//! to read out and present a frame.
//!
//! PIO register groups implemented:
//! - CRT Controller (CRTC):      index 0x3D4 / data 0x3D5   (25 indexed regs)
//! - Sequencer:                  index 0x3C4 / data 0x3C5   (5 indexed regs)
//! - Graphics Controller (GC):   index 0x3CE / data 0x3CF   (9 indexed regs)
//! - Attribute Controller (AC):  combined index/data 0x3C0  (21 indexed regs,
//!                               address/data flip-flop)
//! - Miscellaneous Output:       write 0x3C2 / read 0x3CC
//! - Input Status Register 1:    read 0x3DA (also resets the AC flip-flop)
//! - Input Status Register 0:    read 0x3C2
//! - DAC (palette):              write index 0x3C8 / read index 0x3C7 /
//!                               data 0x3C9 (256 entries x RGB, 6 bits/channel)
//! - DAC pixel mask:             0x3C6
//!
//! Note: The CRTC and Input Status 1 ports are mapped at the color (0x3Dx)
//! addresses, which is the configuration selected when Misc Output bit 0
//! (I/O address select) is set. The monochrome aliases (0x3Bx) are not exposed
//! here.

use super::bus::{IoDevice, MmioDevice};

/// Number of CRT Controller indexed registers (0x00..=0x18).
pub const CRTC_REG_COUNT: usize = 0x19;
/// Number of Sequencer indexed registers (0x00..=0x04).
pub const SEQ_REG_COUNT: usize = 0x05;
/// Number of Graphics Controller indexed registers (0x00..=0x08).
pub const GC_REG_COUNT: usize = 0x09;
/// Number of Attribute Controller indexed registers (0x00..=0x14).
pub const ATTR_REG_COUNT: usize = 0x15;
/// Number of DAC palette entries.
pub const DAC_ENTRIES: usize = 256;

/// Base address of the legacy VGA memory aperture.
pub const VGA_MEM_BASE: u64 = 0xA_0000;
/// End (exclusive) of the legacy VGA memory aperture.
pub const VGA_MEM_END: u64 = 0xC_0000;
/// Size of the backing framebuffer (0xA0000..0xC0000 == 128 KiB).
pub const VGA_MEM_SIZE: usize = (VGA_MEM_END - VGA_MEM_BASE) as usize;

/// Offset within the aperture of the text-mode buffer (0xB8000).
pub const TEXT_BUFFER_OFFSET: usize = (0xB_8000 - VGA_MEM_BASE) as usize;

// PIO port addresses.
const PORT_ATTR_ADDR_DATA: u16 = 0x3C0;
const PORT_ATTR_DATA_READ: u16 = 0x3C1;
const PORT_INPUT_STATUS_0: u16 = 0x3C2; // read
const PORT_MISC_OUTPUT_WRITE: u16 = 0x3C2; // write
const PORT_SEQ_ADDR: u16 = 0x3C4;
const PORT_SEQ_DATA: u16 = 0x3C5;
const PORT_DAC_PEL_MASK: u16 = 0x3C6;
const PORT_DAC_READ_INDEX: u16 = 0x3C7; // write to set read index
const PORT_DAC_WRITE_INDEX: u16 = 0x3C8; // write to set write index (also readable)
const PORT_DAC_DATA: u16 = 0x3C9;
const PORT_MISC_OUTPUT_READ: u16 = 0x3CC; // read
const PORT_GC_ADDR: u16 = 0x3CE;
const PORT_GC_DATA: u16 = 0x3CF;
const PORT_CRTC_ADDR: u16 = 0x3D4;
const PORT_CRTC_DATA: u16 = 0x3D5;
const PORT_INPUT_STATUS_1: u16 = 0x3DA; // read (resets attr flip-flop)

/// I/O port range covering all VGA PIO registers used here (0x3C0..=0x3DA).
pub const VGA_IO_BASE: u16 = 0x3C0;
pub const VGA_IO_LEN: u16 = 0x3DB - 0x3C0; // 0x3C0..=0x3DA inclusive

/// One DAC palette entry: red, green, blue, each 6-bit (0..=63).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DacEntry {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// State of the DAC read/write data sequencer.
///
/// Accesses to the data port (0x3C9) walk through the R, G, B subcomponents of
/// the currently selected palette entry, advancing the entry index after the
/// blue component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DacComponent {
    Red,
    Green,
    Blue,
}

pub struct Vga {
    // CRT Controller
    crtc_index: u8,
    crtc_regs: [u8; CRTC_REG_COUNT],

    // Sequencer
    seq_index: u8,
    seq_regs: [u8; SEQ_REG_COUNT],

    // Graphics Controller
    gc_index: u8,
    gc_regs: [u8; GC_REG_COUNT],

    // Attribute Controller
    attr_index: u8,
    attr_regs: [u8; ATTR_REG_COUNT],
    /// Attribute controller address/data flip-flop.
    /// `false` => next 0x3C0 write is an index; `true` => next is data.
    attr_flip_flop: bool,
    /// Bit 5 of the attribute address register (palette address source).
    attr_palette_enable: bool,

    // Miscellaneous Output register (write 0x3C2 / read 0x3CC).
    misc_output: u8,

    // Feature Control register (read 0x3CA on real hardware; tracked for completeness).
    feature_control: u8,

    // DAC (palette) state.
    dac_palette: [DacEntry; DAC_ENTRIES],
    dac_pel_mask: u8,
    dac_write_index: u8,
    dac_read_index: u8,
    dac_write_component: DacComponent,
    dac_read_component: DacComponent,
    /// Scratch buffers holding the partial RGB triple between sub-accesses.
    dac_write_latch: DacEntry,

    /// Input Status Register 1 toggling bits (vertical retrace / display enable).
    /// Software polls this; we toggle it on each read so polling loops terminate.
    input_status_1_toggle: u8,

    // Backing video memory for 0xA0000-0xBFFFF.
    framebuffer: Vec<u8>,
}

impl Vga {
    pub fn new() -> Self {
        Vga {
            crtc_index: 0,
            crtc_regs: [0; CRTC_REG_COUNT],
            seq_index: 0,
            seq_regs: [0; SEQ_REG_COUNT],
            gc_index: 0,
            gc_regs: [0; GC_REG_COUNT],
            attr_index: 0,
            attr_regs: [0; ATTR_REG_COUNT],
            attr_flip_flop: false,
            attr_palette_enable: false,
            // Default: color emulation, RAM enable, 25 MHz clock (typical BIOS value).
            misc_output: 0x01,
            feature_control: 0,
            dac_palette: [DacEntry::default(); DAC_ENTRIES],
            dac_pel_mask: 0xFF,
            dac_write_index: 0,
            dac_read_index: 0,
            dac_write_component: DacComponent::Red,
            dac_read_component: DacComponent::Red,
            dac_write_latch: DacEntry::default(),
            input_status_1_toggle: 0,
            framebuffer: vec![0u8; VGA_MEM_SIZE],
        }
    }

    // ---- Accessors for renderer / MMIO integration ----

    /// Returns a reference to the full backing video memory buffer
    /// (covers 0xA0000-0xBFFFF, 128 KiB).
    pub fn framebuffer(&self) -> &[u8] {
        &self.framebuffer
    }

    /// Returns a mutable reference to the full backing video memory buffer.
    pub fn framebuffer_mut(&mut self) -> &mut [u8] {
        &mut self.framebuffer
    }

    /// Returns the current DAC palette (256 RGB6 entries).
    pub fn palette(&self) -> &[DacEntry; DAC_ENTRIES] {
        &self.dac_palette
    }

    /// Reads a single byte from VGA video memory at an absolute address in the
    /// 0xA0000-0xBFFFF range. Out-of-range reads return 0xFF.
    pub fn mem_read(&self, addr: u64) -> u8 {
        if addr >= VGA_MEM_BASE && addr < VGA_MEM_END {
            self.framebuffer[(addr - VGA_MEM_BASE) as usize]
        } else {
            0xFF
        }
    }

    /// Writes a single byte to VGA video memory at an absolute address in the
    /// 0xA0000-0xBFFFF range. Out-of-range writes are ignored.
    pub fn mem_write(&mut self, addr: u64, value: u8) {
        if addr >= VGA_MEM_BASE && addr < VGA_MEM_END {
            self.framebuffer[(addr - VGA_MEM_BASE) as usize] = value;
        }
    }

    /// Reads a character cell (character byte + attribute byte) from the text
    /// buffer at 0xB8000. `cell` is the cell index (row * columns + column).
    /// Returns `(character, attribute)`.
    pub fn text_read_cell(&self, cell: usize) -> (u8, u8) {
        let off = TEXT_BUFFER_OFFSET + cell * 2;
        if off + 1 < self.framebuffer.len() {
            (self.framebuffer[off], self.framebuffer[off + 1])
        } else {
            (0, 0)
        }
    }

    /// Writes a character cell (character + attribute) to the text buffer at
    /// 0xB8000.
    pub fn text_write_cell(&mut self, cell: usize, ch: u8, attr: u8) {
        let off = TEXT_BUFFER_OFFSET + cell * 2;
        if off + 1 < self.framebuffer.len() {
            self.framebuffer[off] = ch;
            self.framebuffer[off + 1] = attr;
        }
    }

    // ---- Indexed register read helpers ----

    fn crtc_read(&self) -> u8 {
        self.crtc_regs
            .get(self.crtc_index as usize)
            .copied()
            .unwrap_or(0)
    }

    fn crtc_write(&mut self, value: u8) {
        if let Some(slot) = self.crtc_regs.get_mut(self.crtc_index as usize) {
            *slot = value;
        }
    }

    fn seq_read(&self) -> u8 {
        self.seq_regs
            .get(self.seq_index as usize)
            .copied()
            .unwrap_or(0)
    }

    fn seq_write(&mut self, value: u8) {
        if let Some(slot) = self.seq_regs.get_mut(self.seq_index as usize) {
            *slot = value;
        }
    }

    fn gc_read(&self) -> u8 {
        self.gc_regs
            .get(self.gc_index as usize)
            .copied()
            .unwrap_or(0)
    }

    fn gc_write(&mut self, value: u8) {
        if let Some(slot) = self.gc_regs.get_mut(self.gc_index as usize) {
            *slot = value;
        }
    }

    fn attr_data_read(&self) -> u8 {
        self.attr_regs
            .get(self.attr_index as usize)
            .copied()
            .unwrap_or(0)
    }

    // ---- Attribute controller (0x3C0) handling ----

    fn attr_write(&mut self, value: u8) {
        if !self.attr_flip_flop {
            // Address phase: low 5 bits select the index; bit 5 controls the
            // palette address source (display enable for the internal palette).
            self.attr_index = value & 0x1F;
            self.attr_palette_enable = (value & 0x20) != 0;
        } else {
            // Data phase: store into the selected attribute register.
            if let Some(slot) = self.attr_regs.get_mut(self.attr_index as usize) {
                *slot = value;
            }
        }
        // Each write toggles the flip-flop.
        self.attr_flip_flop = !self.attr_flip_flop;
    }

    /// Read of port 0x3C0 returns the current attribute address register value.
    fn attr_addr_read(&self) -> u8 {
        (self.attr_index & 0x1F) | if self.attr_palette_enable { 0x20 } else { 0 }
    }

    fn reset_attr_flip_flop(&mut self) {
        self.attr_flip_flop = false;
    }

    // ---- DAC (palette) handling ----

    fn dac_data_write(&mut self, value: u8) {
        let v = value & 0x3F; // 6-bit channels
        match self.dac_write_component {
            DacComponent::Red => {
                self.dac_write_latch.r = v;
                self.dac_write_component = DacComponent::Green;
            }
            DacComponent::Green => {
                self.dac_write_latch.g = v;
                self.dac_write_component = DacComponent::Blue;
            }
            DacComponent::Blue => {
                self.dac_write_latch.b = v;
                // Full triple complete: commit and advance to next entry.
                self.dac_palette[self.dac_write_index as usize] = self.dac_write_latch;
                self.dac_write_index = self.dac_write_index.wrapping_add(1);
                self.dac_write_component = DacComponent::Red;
            }
        }
    }

    fn dac_data_read(&mut self) -> u8 {
        let entry = self.dac_palette[self.dac_read_index as usize];
        let value = match self.dac_read_component {
            DacComponent::Red => {
                self.dac_read_component = DacComponent::Green;
                entry.r
            }
            DacComponent::Green => {
                self.dac_read_component = DacComponent::Blue;
                entry.g
            }
            DacComponent::Blue => {
                self.dac_read_index = self.dac_read_index.wrapping_add(1);
                self.dac_read_component = DacComponent::Red;
                entry.b
            }
        };
        value & 0x3F
    }
}

impl Default for Vga {
    fn default() -> Self {
        Vga::new()
    }
}

impl IoDevice for Vga {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            PORT_ATTR_ADDR_DATA => self.attr_addr_read(),
            PORT_ATTR_DATA_READ => self.attr_data_read(),
            PORT_INPUT_STATUS_0 => {
                // Input Status Register 0. Bit 7 = CRT interrupt (not modeled).
                0x00
            }
            PORT_SEQ_ADDR => self.seq_index,
            PORT_SEQ_DATA => self.seq_read(),
            PORT_DAC_PEL_MASK => self.dac_pel_mask,
            PORT_DAC_READ_INDEX => {
                // DAC state register: 0b00 == write mode, 0b11 == read mode.
                // Report read mode since 0x3C7 was used to set the read index.
                0x03
            }
            PORT_DAC_WRITE_INDEX => self.dac_write_index,
            PORT_DAC_DATA => self.dac_data_read(),
            PORT_MISC_OUTPUT_READ => self.misc_output,
            // 0x3CA: Feature Control read.
            0x3CA => self.feature_control,
            PORT_GC_ADDR => self.gc_index,
            PORT_GC_DATA => self.gc_read(),
            PORT_CRTC_ADDR => self.crtc_index,
            PORT_CRTC_DATA => self.crtc_read(),
            PORT_INPUT_STATUS_1 => {
                // Reading Input Status 1 resets the attribute flip-flop to the
                // address (index) phase.
                self.reset_attr_flip_flop();
                // Toggle the display-enable (bit 0) and vertical-retrace (bit 3)
                // status bits so that polling loops in guest software make
                // progress and eventually observe both states.
                self.input_status_1_toggle = self.input_status_1_toggle.wrapping_add(1);
                let de = (self.input_status_1_toggle & 0x01) != 0;
                let vr = (self.input_status_1_toggle & 0x08) != 0;
                let mut status = 0u8;
                if de {
                    status |= 0x01; // Display Enable (inverted on hw, simplified here)
                }
                if vr {
                    status |= 0x08; // Vertical Retrace
                }
                status
            }
            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            PORT_ATTR_ADDR_DATA => self.attr_write(value),
            PORT_MISC_OUTPUT_WRITE => self.misc_output = value,
            PORT_SEQ_ADDR => self.seq_index = value,
            PORT_SEQ_DATA => self.seq_write(value),
            PORT_DAC_PEL_MASK => self.dac_pel_mask = value,
            PORT_DAC_READ_INDEX => {
                // Setting the read index resets the read component sequencer.
                self.dac_read_index = value;
                self.dac_read_component = DacComponent::Red;
            }
            PORT_DAC_WRITE_INDEX => {
                // Setting the write index resets the write component sequencer.
                self.dac_write_index = value;
                self.dac_write_component = DacComponent::Red;
                self.dac_write_latch = DacEntry::default();
            }
            PORT_DAC_DATA => self.dac_data_write(value),
            // 0x3CA: Feature Control write (color address).
            0x3CA => self.feature_control = value,
            PORT_GC_ADDR => self.gc_index = value,
            PORT_GC_DATA => self.gc_write(value),
            PORT_CRTC_ADDR => self.crtc_index = value,
            PORT_CRTC_DATA => self.crtc_write(value),
            // Input Status 1 (0x3DA) is read-only; a write here is the Feature
            // Control register alias on some chipsets. Treat as feature control.
            PORT_INPUT_STATUS_1 => self.feature_control = value,
            _ => {}
        }
    }
}

/// MMIO hookup for the legacy VGA aperture (0xA0000-0xBFFFF). This provides a
/// simple linear view of the backing framebuffer; planar latch/ALU logic of
/// real VGA is out of scope here but the raw memory is preserved so a renderer
/// can interpret it according to the indexed registers.
impl MmioDevice for Vga {
    fn read(&mut self, addr: u64, data: &mut [u8]) {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte = self.mem_read(addr.wrapping_add(i as u64));
        }
    }

    fn write(&mut self, addr: u64, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem_write(addr.wrapping_add(i as u64), byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // PIO helpers that disambiguate between the IoDevice and MmioDevice trait
    // methods (both named `read`/`write`) for the `Vga` type.
    fn pio_write(vga: &mut Vga, port: u16, value: u8) {
        IoDevice::write(vga, port, value);
    }

    fn pio_read(vga: &mut Vga, port: u16) -> u8 {
        IoDevice::read(vga, port)
    }

    #[test]
    fn crtc_index_data_round_trip() {
        let mut vga = Vga::new();
        // Select CRTC index 0x0C (start address high), write data, read back.
        pio_write(&mut vga, PORT_CRTC_ADDR, 0x0C);
        assert_eq!(pio_read(&mut vga, PORT_CRTC_ADDR), 0x0C);
        pio_write(&mut vga, PORT_CRTC_DATA, 0xAB);
        assert_eq!(pio_read(&mut vga, PORT_CRTC_DATA), 0xAB);

        // A different index must not alias the previous register.
        pio_write(&mut vga, PORT_CRTC_ADDR, 0x0D);
        pio_write(&mut vga, PORT_CRTC_DATA, 0xCD);
        assert_eq!(pio_read(&mut vga, PORT_CRTC_DATA), 0xCD);

        pio_write(&mut vga, PORT_CRTC_ADDR, 0x0C);
        assert_eq!(pio_read(&mut vga, PORT_CRTC_DATA), 0xAB);
    }

    #[test]
    fn sequencer_indexed_regs() {
        let mut vga = Vga::new();
        pio_write(&mut vga, PORT_SEQ_ADDR, 0x02); // Map Mask
        assert_eq!(pio_read(&mut vga, PORT_SEQ_ADDR), 0x02);
        pio_write(&mut vga, PORT_SEQ_DATA, 0x0F);
        assert_eq!(pio_read(&mut vga, PORT_SEQ_DATA), 0x0F);

        pio_write(&mut vga, PORT_SEQ_ADDR, 0x04); // Memory Mode
        pio_write(&mut vga, PORT_SEQ_DATA, 0x06);
        assert_eq!(pio_read(&mut vga, PORT_SEQ_DATA), 0x06);

        // Out-of-range index reads as 0 and ignores writes.
        pio_write(&mut vga, PORT_SEQ_ADDR, 0x40);
        pio_write(&mut vga, PORT_SEQ_DATA, 0xFF);
        assert_eq!(pio_read(&mut vga, PORT_SEQ_DATA), 0x00);
    }

    #[test]
    fn graphics_controller_indexed_regs() {
        let mut vga = Vga::new();
        pio_write(&mut vga, PORT_GC_ADDR, 0x05); // Graphics Mode
        assert_eq!(pio_read(&mut vga, PORT_GC_ADDR), 0x05);
        pio_write(&mut vga, PORT_GC_DATA, 0x40); // 256-color mode bit
        assert_eq!(pio_read(&mut vga, PORT_GC_DATA), 0x40);

        pio_write(&mut vga, PORT_GC_ADDR, 0x06); // Misc Graphics
        pio_write(&mut vga, PORT_GC_DATA, 0x05);
        assert_eq!(pio_read(&mut vga, PORT_GC_DATA), 0x05);

        // Earlier register retained.
        pio_write(&mut vga, PORT_GC_ADDR, 0x05);
        assert_eq!(pio_read(&mut vga, PORT_GC_DATA), 0x40);
    }

    #[test]
    fn attribute_controller_flip_flop() {
        let mut vga = Vga::new();

        // Reset the flip-flop via Input Status 1 (0x3DA) -> address phase next.
        pio_read(&mut vga, PORT_INPUT_STATUS_1);

        // First 0x3C0 write = index (with palette-enable bit set in bit 5).
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x20 | 0x10); // index 0x10 (Mode Control)
        // Address register read reflects index and palette-enable bit.
        assert_eq!(pio_read(&mut vga, PORT_ATTR_ADDR_DATA), 0x20 | 0x10);

        // Second 0x3C0 write = data for index 0x10.
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x41);
        // Read back via the dedicated data-read port 0x3C1.
        assert_eq!(pio_read(&mut vga, PORT_ATTR_DATA_READ), 0x41);

        // Flip-flop is back to address phase; write a new index.
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x11); // index 0x11 (overscan), palette disabled
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x22); // data
        assert_eq!(pio_read(&mut vga, PORT_ATTR_DATA_READ), 0x22);

        // Reading 0x3DA mid-sequence must reset the flip-flop to address phase.
        // Put it into data phase first by writing an index...
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x12); // index 0x12 -> now data phase
        // ...then reset.
        pio_read(&mut vga, PORT_INPUT_STATUS_1);
        // Next write is treated as an index again (not data for 0x12).
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x13); // index 0x13
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x77); // data for 0x13
        assert_eq!(pio_read(&mut vga, PORT_ATTR_DATA_READ), 0x77);

        // Index 0x12 should be unchanged from its default (0) because the
        // 0x22-style write never landed there after the reset.
        pio_read(&mut vga, PORT_INPUT_STATUS_1);
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x12);
        assert_eq!(pio_read(&mut vga, PORT_ATTR_DATA_READ), 0x00);
    }

    #[test]
    fn dac_palette_write_readback() {
        let mut vga = Vga::new();

        // Set write index to 5, write two RGB triples (entries 5 and 6).
        pio_write(&mut vga, PORT_DAC_WRITE_INDEX, 5);
        // Entry 5: R,G,B
        pio_write(&mut vga, PORT_DAC_DATA, 0x10);
        pio_write(&mut vga, PORT_DAC_DATA, 0x20);
        pio_write(&mut vga, PORT_DAC_DATA, 0x30);
        // Entry 6: R,G,B
        pio_write(&mut vga, PORT_DAC_DATA, 0x01);
        pio_write(&mut vga, PORT_DAC_DATA, 0x02);
        pio_write(&mut vga, PORT_DAC_DATA, 0x03);

        // Write index should have auto-advanced past both entries.
        assert_eq!(pio_read(&mut vga, PORT_DAC_WRITE_INDEX), 7);

        // Read them back starting at index 5.
        pio_write(&mut vga, PORT_DAC_READ_INDEX, 5);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x10);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x20);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x30);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x01);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x02);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x03);

        // Verify channel values are masked to 6 bits on write.
        pio_write(&mut vga, PORT_DAC_WRITE_INDEX, 0);
        pio_write(&mut vga, PORT_DAC_DATA, 0xFF); // R -> 0x3F
        pio_write(&mut vga, PORT_DAC_DATA, 0xC0); // G -> 0x00
        pio_write(&mut vga, PORT_DAC_DATA, 0x7E); // B -> 0x3E
        pio_write(&mut vga, PORT_DAC_READ_INDEX, 0);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x3F);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x00);
        assert_eq!(pio_read(&mut vga, PORT_DAC_DATA), 0x3E);
    }

    #[test]
    fn misc_output_read_after_write() {
        let mut vga = Vga::new();
        pio_write(&mut vga, PORT_MISC_OUTPUT_WRITE, 0x67); // common BIOS value
        assert_eq!(pio_read(&mut vga, PORT_MISC_OUTPUT_READ), 0x67);

        pio_write(&mut vga, PORT_MISC_OUTPUT_WRITE, 0xE3);
        assert_eq!(pio_read(&mut vga, PORT_MISC_OUTPUT_READ), 0xE3);
    }

    #[test]
    fn framebuffer_sizing_and_mem_access() {
        let mut vga = Vga::new();
        assert_eq!(vga.framebuffer().len(), VGA_MEM_SIZE);
        assert_eq!(VGA_MEM_SIZE, 0x20000); // 128 KiB

        // Linear access at the graphics base 0xA0000.
        vga.mem_write(0xA_0000, 0x55);
        assert_eq!(vga.mem_read(0xA_0000), 0x55);
        assert_eq!(vga.framebuffer()[0], 0x55);

        // Access at the very end of the aperture.
        vga.mem_write(VGA_MEM_END - 1, 0xAA);
        assert_eq!(vga.mem_read(VGA_MEM_END - 1), 0xAA);

        // Out-of-range reads return 0xFF and writes are ignored.
        assert_eq!(vga.mem_read(VGA_MEM_END), 0xFF);
        vga.mem_write(VGA_MEM_END, 0x12); // no panic, no effect
    }

    #[test]
    fn text_buffer_cell_access() {
        let mut vga = Vga::new();
        // 'A' (0x41) with attribute 0x0F (white on black) at cell 0 (B8000).
        vga.text_write_cell(0, 0x41, 0x0F);
        let (ch, attr) = vga.text_read_cell(0);
        assert_eq!(ch, 0x41);
        assert_eq!(attr, 0x0F);

        // Verify it landed at the 0xB8000 offset in raw memory.
        assert_eq!(vga.mem_read(0xB_8000), 0x41);
        assert_eq!(vga.mem_read(0xB_8001), 0x0F);

        // Second cell.
        vga.text_write_cell(1, 0x42, 0x1C);
        let (ch1, attr1) = vga.text_read_cell(1);
        assert_eq!(ch1, 0x42);
        assert_eq!(attr1, 0x1C);
    }

    #[test]
    fn mmio_linear_round_trip() {
        let mut vga = Vga::new();
        // Write a multi-byte buffer through the MmioDevice impl.
        MmioDevice::write(&mut vga, 0xB_8000, &[b'H', 0x07, b'i', 0x07]);
        let mut buf = [0u8; 4];
        MmioDevice::read(&mut vga, 0xB_8000, &mut buf);
        assert_eq!(buf, [b'H', 0x07, b'i', 0x07]);
    }

    #[test]
    fn dac_pel_mask_round_trip() {
        let mut vga = Vga::new();
        assert_eq!(pio_read(&mut vga, PORT_DAC_PEL_MASK), 0xFF); // default
        pio_write(&mut vga, PORT_DAC_PEL_MASK, 0x3F);
        assert_eq!(pio_read(&mut vga, PORT_DAC_PEL_MASK), 0x3F);
    }

    #[test]
    fn input_status_1_resets_flip_flop_and_progresses() {
        let mut vga = Vga::new();
        // Put the AC into data phase.
        pio_read(&mut vga, PORT_INPUT_STATUS_1);
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x05); // index -> data phase next
        // Reset.
        pio_read(&mut vga, PORT_INPUT_STATUS_1);
        // A subsequent write is an index again.
        pio_write(&mut vga, PORT_ATTR_ADDR_DATA, 0x0A);
        assert_eq!(pio_read(&mut vga, PORT_ATTR_ADDR_DATA) & 0x1F, 0x0A);

        // The status byte should eventually show vertical retrace across polls.
        let mut saw_vr = false;
        for _ in 0..16 {
            if pio_read(&mut vga, PORT_INPUT_STATUS_1) & 0x08 != 0 {
                saw_vr = true;
                break;
            }
        }
        assert!(saw_vr);
    }
}
