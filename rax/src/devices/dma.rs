//! Intel 8237A Programmable DMA Controller pair emulation.
//!
//! The legacy PC/AT contains two cascaded 8237A DMA controllers providing
//! eight DMA channels:
//! - Controller 1 (master): channels 0-3, 8-bit transfers, ports 0x00-0x0F
//! - Controller 2 (slave):  channels 4-7, 16-bit transfers, ports 0xC0-0xDF
//!
//! Channel 4 is used to cascade controller 1 into controller 2 and is not
//! available for general use, but the registers are modeled uniformly.
//!
//! Each channel has:
//! - A current address register and current word-count register, each accessed
//!   one byte at a time through a shared high/low byte pointer (the
//!   "byte-pointer flip-flop").
//! - A base address register and base word-count register, latched whenever the
//!   corresponding current register is written (used for autoinitialization).
//! - A mode register.
//! - A page register (the upper bits of the physical address), located in the
//!   separate page-register file at ports 0x80-0x8F (74LS612 mapper).
//!
//! Actual data transfer to/from devices is out of scope: this module faithfully
//! models the programmable register state and exposes getters for inspection.
//!
//! ## Port layout
//!
//! Controller 1 (8-bit) registers occupy consecutive byte ports 0x00-0x0F.
//! Controller 2 (16-bit) registers occupy ports 0xC0-0xDF but are spaced two
//! ports apart (only even offsets carry meaning), reflecting the wiring of the
//! 16-bit controller onto the ISA address bus.
//!
//! Channel address/count register pairs:
//! - Ctrl 1: 0x00..0x07 -> (ch0 addr, ch0 cnt, ch1 addr, ch1 cnt, ...)
//! - Ctrl 2: 0xC0..0xCF -> (ch4 addr, ch4 cnt, ch5 addr, ch5 cnt, ...)
//!
//! Command ports:
//! - Status (read) / Command (write):       0x08 / 0xD0
//! - Request:                                0x09 / 0xD2
//! - Single mask bit:                        0x0A / 0xD4
//! - Mode:                                   0x0B / 0xD6
//! - Clear byte-pointer flip-flop:           0x0C / 0xD8
//! - Master clear (read = temporary reg):    0x0D / 0xDA
//! - Clear mask register:                    0x0E / 0xDC
//! - Write all mask bits:                    0x0F / 0xDE
//!
//! Page registers (0x80-0x8F): the historical I/O addresses map to channels in
//! a non-linear fashion. The relevant ones are:
//! - 0x87 -> channel 0, 0x83 -> channel 1, 0x81 -> channel 2, 0x82 -> channel 3
//! - 0x8B -> channel 5, 0x89 -> channel 6, 0x8A -> channel 7
//! - 0x8F -> channel 4 (refresh / cascade page)

use super::bus::IoDevice;

/// Per-channel programmable state.
///
/// Address and count registers store the 16-bit value latched into the chip.
/// For the 8-bit controller these are byte addresses/counts; for the 16-bit
/// controller they are word addresses/counts (the hardware shifts them when
/// driving the physical bus, which is not modeled here).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Channel {
    /// Current address register (decremented during a real transfer).
    pub current_address: u16,
    /// Current word-count register.
    pub current_count: u16,
    /// Base address register, reloaded on autoinitialize.
    pub base_address: u16,
    /// Base word-count register, reloaded on autoinitialize.
    pub base_count: u16,
    /// Mode register byte (mode bits 7:6, decrement 5, autoinit 4,
    /// transfer type 3:2, channel select 1:0).
    pub mode: u8,
    /// Page register (upper physical address byte).
    pub page: u8,
    /// True if the channel is masked (DREQ ignored).
    pub masked: bool,
}

/// A single 8237A controller (4 channels).
struct Controller {
    channels: [Channel; 4],
    /// Byte-pointer flip-flop: false selects the low byte, true the high byte.
    /// Shared across all address/count accesses on this controller.
    flip_flop: bool,
    /// Command register (write-only on hardware; stored for inspection).
    command: u8,
    /// Status register: bits 3:0 = terminal-count reached, bits 7:4 = request.
    /// Reading the status register clears the terminal-count bits.
    status: u8,
    /// Request register bits (software DMA requests).
    request: u8,
    /// Temporary register, returned when reading the master-clear port.
    temporary: u8,
}

impl Controller {
    fn new() -> Self {
        Controller {
            channels: [Channel::default(); 4],
            // All channels start masked after a master clear / power-up.
            flip_flop: false,
            command: 0,
            status: 0,
            request: 0,
            temporary: 0,
        }
    }

    /// Master clear: equivalent to a hardware reset of the controller.
    /// Clears command, status, request and the flip-flop, and masks all
    /// channels. Address/count/mode/page registers are left intact, matching
    /// the 8237A behavior where master clear does not zero the channel data
    /// registers but does set the mask bits and clear the flip-flop.
    fn master_clear(&mut self) {
        self.flip_flop = false;
        self.command = 0;
        self.status = 0;
        self.request = 0;
        self.temporary = 0;
        for ch in &mut self.channels {
            ch.masked = true;
        }
    }

    /// Read the next byte of a 16-bit register, advancing the flip-flop.
    fn read_word_byte(&mut self, value: u16) -> u8 {
        let byte = if self.flip_flop {
            (value >> 8) as u8
        } else {
            (value & 0xFF) as u8
        };
        self.flip_flop = !self.flip_flop;
        byte
    }

    /// Write the next byte of a 16-bit register, advancing the flip-flop.
    /// Returns the updated 16-bit value.
    fn write_word_byte(&mut self, current: u16, value: u8) -> u16 {
        let updated = if self.flip_flop {
            (current & 0x00FF) | ((value as u16) << 8)
        } else {
            (current & 0xFF00) | (value as u16)
        };
        self.flip_flop = !self.flip_flop;
        updated
    }
}

/// The legacy DMA controller pair plus the page-register file.
pub struct Dma {
    /// Controller 1 (channels 0-3).
    c1: Controller,
    /// Controller 2 (channels 4-7).
    c2: Controller,
}

impl Default for Dma {
    fn default() -> Self {
        Self::new()
    }
}

impl Dma {
    pub fn new() -> Self {
        Dma {
            c1: Controller::new(),
            c2: Controller::new(),
        }
    }

    /// Get an immutable view of a channel (0-7).
    pub fn channel(&self, channel: usize) -> &Channel {
        if channel < 4 {
            &self.c1.channels[channel]
        } else {
            &self.c2.channels[channel - 4]
        }
    }

    /// Current address register for a channel (0-7).
    pub fn current_address(&self, channel: usize) -> u16 {
        self.channel(channel).current_address
    }

    /// Current word-count register for a channel (0-7).
    pub fn current_count(&self, channel: usize) -> u16 {
        self.channel(channel).current_count
    }

    /// Base address register for a channel (0-7).
    pub fn base_address(&self, channel: usize) -> u16 {
        self.channel(channel).base_address
    }

    /// Base word-count register for a channel (0-7).
    pub fn base_count(&self, channel: usize) -> u16 {
        self.channel(channel).base_count
    }

    /// Mode register byte for a channel (0-7).
    pub fn mode(&self, channel: usize) -> u8 {
        self.channel(channel).mode
    }

    /// Page register byte for a channel (0-7).
    pub fn page(&self, channel: usize) -> u8 {
        self.channel(channel).page
    }

    /// Whether the channel (0-7) is masked.
    pub fn is_masked(&self, channel: usize) -> bool {
        self.channel(channel).masked
    }

    /// State of the byte-pointer flip-flop for a controller (0 or 1).
    pub fn flip_flop(&self, controller: usize) -> bool {
        if controller == 0 {
            self.c1.flip_flop
        } else {
            self.c2.flip_flop
        }
    }

    /// Compose the full 32-bit physical address from page + current address.
    /// For the 8-bit controller this is `page << 16 | address`. For the 16-bit
    /// controller the word address is shifted left by one and the page supplies
    /// bits 23:16 (bit 16 of the address is forced from the page low bit being
    /// ignored on real hardware); here we apply the standard `page << 16 |
    /// (address << 1)` mapping.
    pub fn physical_address(&self, channel: usize) -> u32 {
        let ch = self.channel(channel);
        if channel < 4 {
            ((ch.page as u32) << 16) | (ch.current_address as u32)
        } else {
            ((ch.page as u32) << 16) | ((ch.current_address as u32) << 1)
        }
    }

    // ---- Page register file (0x80-0x8F) --------------------------------

    /// Map a page I/O port to a channel index (0-7), if it corresponds to a
    /// DMA channel page register. Returns None for the reserved/refresh ports.
    fn page_port_to_channel(port: u16) -> Option<usize> {
        match port {
            0x87 => Some(0),
            0x83 => Some(1),
            0x81 => Some(2),
            0x82 => Some(3),
            0x8F => Some(4),
            0x8B => Some(5),
            0x89 => Some(6),
            0x8A => Some(7),
            _ => None,
        }
    }

    fn write_page(&mut self, port: u16, value: u8) {
        if let Some(ch) = Self::page_port_to_channel(port) {
            if ch < 4 {
                self.c1.channels[ch].page = value;
            } else {
                self.c2.channels[ch - 4].page = value;
            }
        }
        // Other ports in 0x80-0x8F are scratch/unused page regs; ignore.
    }

    fn read_page(&self, port: u16) -> u8 {
        match Self::page_port_to_channel(port) {
            Some(ch) if ch < 4 => self.c1.channels[ch].page,
            Some(ch) => self.c2.channels[ch - 4].page,
            None => 0xFF,
        }
    }

    // ---- Generic controller register access ----------------------------

    /// Handle a write to a channel address/count register pair.
    /// `index` is 0..=7 within the controller's 8-port window where even
    /// values are address registers and odd values are count registers.
    fn write_channel_reg(ctrl: &mut Controller, index: usize, value: u8) {
        let ch = index / 2;
        let is_count = (index & 1) == 1;
        if is_count {
            let updated = ctrl.write_word_byte(ctrl.channels[ch].current_count, value);
            ctrl.channels[ch].current_count = updated;
            ctrl.channels[ch].base_count = updated;
        } else {
            let updated = ctrl.write_word_byte(ctrl.channels[ch].current_address, value);
            ctrl.channels[ch].current_address = updated;
            ctrl.channels[ch].base_address = updated;
        }
    }

    fn read_channel_reg(ctrl: &mut Controller, index: usize) -> u8 {
        let ch = index / 2;
        let is_count = (index & 1) == 1;
        if is_count {
            let v = ctrl.channels[ch].current_count;
            ctrl.read_word_byte(v)
        } else {
            let v = ctrl.channels[ch].current_address;
            ctrl.read_word_byte(v)
        }
    }

    /// Single-mask-bit command: bits 1:0 select the channel within the
    /// controller, bit 2 sets (1) or clears (0) the mask.
    fn write_single_mask(ctrl: &mut Controller, value: u8) {
        let ch = (value & 0x03) as usize;
        let set = (value & 0x04) != 0;
        ctrl.channels[ch].masked = set;
    }

    /// Write-all-mask-bits command: bits 3:0 each mask one channel.
    fn write_all_mask(ctrl: &mut Controller, value: u8) {
        for (i, ch) in ctrl.channels.iter_mut().enumerate() {
            ch.masked = (value & (1 << i)) != 0;
        }
    }

    /// Mode command: bits 1:0 select the channel, the rest is stored verbatim.
    fn write_mode(ctrl: &mut Controller, value: u8) {
        let ch = (value & 0x03) as usize;
        ctrl.channels[ch].mode = value;
    }

    /// Request command: bits 1:0 select the channel, bit 2 set/clear.
    fn write_request(ctrl: &mut Controller, value: u8) {
        let ch = (value & 0x03) as u8;
        if (value & 0x04) != 0 {
            ctrl.request |= 1 << ch;
        } else {
            ctrl.request &= !(1 << ch);
        }
    }

    /// Build the status byte: low nibble = terminal count reached (cleared on
    /// read), high nibble = channel request pending.
    fn read_status(ctrl: &mut Controller) -> u8 {
        let value = (ctrl.status & 0x0F) | ((ctrl.request & 0x0F) << 4);
        // Reading status clears the terminal-count bits (low nibble).
        ctrl.status &= 0xF0;
        value
    }
}

impl IoDevice for Dma {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            // Controller 1 channel registers (0x00-0x07).
            0x00..=0x07 => Self::read_channel_reg(&mut self.c1, port as usize),
            // Controller 1 command/status block.
            0x08 => Self::read_status(&mut self.c1),
            0x09 => 0xFF,              // request register: write-only
            0x0A => 0xFF,              // single mask: write-only
            0x0B => 0xFF,              // mode: write-only
            0x0C => 0xFF,              // clear flip-flop: write-only
            0x0D => self.c1.temporary, // master clear port reads temp register
            0x0E => 0xFF,              // clear mask: write-only
            0x0F => 0xFF,              // write-all-mask: write-only

            // Page registers.
            0x80..=0x8F => self.read_page(port),

            // Controller 2 channel registers (0xC0-0xCF), spaced by 2.
            0xC0..=0xCF => {
                if port & 1 == 0 {
                    let index = ((port - 0xC0) / 2) as usize;
                    Self::read_channel_reg(&mut self.c2, index)
                } else {
                    0xFF
                }
            }
            // Controller 2 command/status block (0xD0-0xDF), spaced by 2.
            0xD0 => Self::read_status(&mut self.c2),
            0xDA => self.c2.temporary,
            0xD2 | 0xD4 | 0xD6 | 0xD8 | 0xDC | 0xDE => 0xFF, // write-only regs
            0xD1 | 0xD3 | 0xD5 | 0xD7 | 0xD9 | 0xDB | 0xDD | 0xDF => 0xFF,

            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            // Controller 1 channel registers (0x00-0x07).
            0x00..=0x07 => Self::write_channel_reg(&mut self.c1, port as usize, value),
            // Controller 1 command/status block.
            0x08 => self.c1.command = value,
            0x09 => Self::write_request(&mut self.c1, value),
            0x0A => Self::write_single_mask(&mut self.c1, value),
            0x0B => Self::write_mode(&mut self.c1, value),
            0x0C => self.c1.flip_flop = false, // clear byte-pointer flip-flop
            0x0D => self.c1.master_clear(),
            0x0E => {
                // Clear mask register: enable all channels.
                for ch in &mut self.c1.channels {
                    ch.masked = false;
                }
            }
            0x0F => Self::write_all_mask(&mut self.c1, value),

            // Page registers.
            0x80..=0x8F => self.write_page(port, value),

            // Controller 2 channel registers (0xC0-0xCF), spaced by 2.
            0xC0..=0xCF => {
                if port & 1 == 0 {
                    let index = ((port - 0xC0) / 2) as usize;
                    Self::write_channel_reg(&mut self.c2, index, value);
                }
            }
            // Controller 2 command/status block (0xD0-0xDF), spaced by 2.
            0xD0 => self.c2.command = value,
            0xD2 => Self::write_request(&mut self.c2, value),
            0xD4 => Self::write_single_mask(&mut self.c2, value),
            0xD6 => Self::write_mode(&mut self.c2, value),
            0xD8 => self.c2.flip_flop = false,
            0xDA => self.c2.master_clear(),
            0xDC => {
                for ch in &mut self.c2.channels {
                    ch.masked = false;
                }
            }
            0xDE => Self::write_all_mask(&mut self.c2, value),

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dma() -> Dma {
        Dma::new()
    }

    #[test]
    fn address_register_low_high_via_flip_flop() {
        let mut d = dma();
        // Flip-flop starts low.
        assert!(!d.flip_flop(0));
        // Write channel 0 address: low byte then high byte.
        d.write(0x00, 0x34);
        assert!(d.flip_flop(0)); // advanced to high
        d.write(0x00, 0x12);
        assert!(!d.flip_flop(0)); // back to low
        assert_eq!(d.current_address(0), 0x1234);
        assert_eq!(d.base_address(0), 0x1234);

        // Read it back via the flip-flop.
        assert_eq!(d.read(0x00), 0x34); // low
        assert_eq!(d.read(0x00), 0x12); // high
    }

    #[test]
    fn count_register_via_flip_flop() {
        let mut d = dma();
        // Channel 1 count register is port 0x03 (index 3).
        d.write(0x03, 0xCD);
        d.write(0x03, 0xAB);
        assert_eq!(d.current_count(1), 0xABCD);
        assert_eq!(d.base_count(1), 0xABCD);
        assert_eq!(d.read(0x03), 0xCD);
        assert_eq!(d.read(0x03), 0xAB);
    }

    #[test]
    fn controller2_16bit_channel_registers() {
        let mut d = dma();
        // Controller 2 register layout (even ports, spaced by 2):
        //   0xC0 = ch4 addr, 0xC2 = ch4 count,
        //   0xC4 = ch5 addr, 0xC6 = ch5 count, ...
        // Channel 5 address register: port 0xC4 (index 2 -> ch1 of ctrl2 addr).
        d.write(0xC4, 0x55);
        d.write(0xC4, 0xAA);
        assert_eq!(d.current_address(5), 0xAA55);
        assert_eq!(d.base_address(5), 0xAA55);
        // Channel 5 count register: port 0xC6 (index 3 -> ch1 of ctrl2 count).
        d.write(0xC6, 0x01);
        d.write(0xC6, 0x02);
        assert_eq!(d.current_count(5), 0x0201);
        // Odd ports between are ignored.
        d.write(0xC3, 0xFF);
        assert_eq!(d.read(0xC3), 0xFF);
    }

    #[test]
    fn mode_register() {
        let mut d = dma();
        // Mode write for controller 1: channel selected by low 2 bits.
        // 0b0100_0110 -> channel 2, single mode, write transfer.
        d.write(0x0B, 0x46);
        assert_eq!(d.mode(2), 0x46);
        // Controller 2 mode at port 0xD6: 0b1000_1001 -> channel 1 of ctrl2 = ch5.
        d.write(0xD6, 0x89);
        assert_eq!(d.mode(5), 0x89);
        // Mode register is write-only.
        assert_eq!(d.read(0x0B), 0xFF);
    }

    #[test]
    fn single_mask_bit() {
        let mut d = dma();
        // After construction channels are unmasked (only master clear masks).
        assert!(!d.is_masked(0));
        // Set mask on channel 2: value bits = ch(2) | set(0x04) = 0x06.
        d.write(0x0A, 0x06);
        assert!(d.is_masked(2));
        assert!(!d.is_masked(0));
        // Clear mask on channel 2: value bits = ch(2) = 0x02.
        d.write(0x0A, 0x02);
        assert!(!d.is_masked(2));

        // Controller 2 single mask at 0xD4: set channel 3 of ctrl2 = ch7.
        d.write(0xD4, 0x07); // ch(3) | set
        assert!(d.is_masked(7));
    }

    #[test]
    fn write_all_mask() {
        let mut d = dma();
        // Mask channels 0 and 2 (bits 0 and 2 = 0b0101 = 0x05).
        d.write(0x0F, 0x05);
        assert!(d.is_masked(0));
        assert!(!d.is_masked(1));
        assert!(d.is_masked(2));
        assert!(!d.is_masked(3));

        // Controller 2: mask channels 4 and 7 (bits 0 and 3 = 0b1001 = 0x09).
        d.write(0xDE, 0x09);
        assert!(d.is_masked(4));
        assert!(!d.is_masked(5));
        assert!(!d.is_masked(6));
        assert!(d.is_masked(7));
    }

    #[test]
    fn clear_mask_register() {
        let mut d = dma();
        d.write(0x0F, 0x0F); // mask all ctrl1 channels
        assert!(d.is_masked(0) && d.is_masked(3));
        d.write(0x0E, 0x00); // clear mask register: enable all
        assert!(!d.is_masked(0));
        assert!(!d.is_masked(1));
        assert!(!d.is_masked(2));
        assert!(!d.is_masked(3));
    }

    #[test]
    fn master_clear_resets_flip_flop_and_masks() {
        let mut d = dma();
        // Advance the flip-flop and unmask everything.
        d.write(0x00, 0x12);
        assert!(d.flip_flop(0));
        d.write(0x0E, 0x00); // unmask all
        assert!(!d.is_masked(0));
        // Master clear controller 1.
        d.write(0x0D, 0x00);
        assert!(!d.flip_flop(0)); // flip-flop reset
        // All channels masked after master clear.
        for ch in 0..4 {
            assert!(d.is_masked(ch));
        }
        // Controller 2 unaffected.
        d.write(0xC0, 0x12);
        assert!(d.flip_flop(1));
        d.write(0xDA, 0x00); // master clear ctrl2
        assert!(!d.flip_flop(1));
        for ch in 4..8 {
            assert!(d.is_masked(ch));
        }
    }

    #[test]
    fn page_registers() {
        let mut d = dma();
        // Page port mapping per spec.
        d.write(0x87, 0xAB); // channel 0
        d.write(0x83, 0xCD); // channel 1
        d.write(0x81, 0xEF); // channel 2
        d.write(0x82, 0x12); // channel 3
        d.write(0x8F, 0x34); // channel 4
        d.write(0x8B, 0x56); // channel 5
        d.write(0x89, 0x78); // channel 6
        d.write(0x8A, 0x9A); // channel 7
        assert_eq!(d.page(0), 0xAB);
        assert_eq!(d.page(1), 0xCD);
        assert_eq!(d.page(2), 0xEF);
        assert_eq!(d.page(3), 0x12);
        assert_eq!(d.page(4), 0x34);
        assert_eq!(d.page(5), 0x56);
        assert_eq!(d.page(6), 0x78);
        assert_eq!(d.page(7), 0x9A);
        // Read back.
        assert_eq!(d.read(0x87), 0xAB);
        assert_eq!(d.read(0x8A), 0x9A);
    }

    #[test]
    fn clear_flip_flop_command() {
        let mut d = dma();
        // Advance flip-flop by writing a single byte.
        d.write(0x00, 0xFF);
        assert!(d.flip_flop(0));
        // Clear byte-pointer flip-flop command.
        d.write(0x0C, 0x00);
        assert!(!d.flip_flop(0));
        // Now the next write goes to the low byte again.
        d.write(0x00, 0x77);
        d.write(0x00, 0x88);
        assert_eq!(d.current_address(0), 0x8877);

        // Controller 2 clear flip-flop at 0xD8.
        d.write(0xC0, 0xFF);
        assert!(d.flip_flop(1));
        d.write(0xD8, 0x00);
        assert!(!d.flip_flop(1));
    }

    #[test]
    fn physical_address_composition() {
        let mut d = dma();
        // Channel 0 (8-bit): page << 16 | address.
        d.write(0x00, 0x00);
        d.write(0x00, 0x10); // address = 0x1000
        d.write(0x87, 0x05); // page = 0x05
        assert_eq!(d.physical_address(0), 0x0005_1000);

        // Channel 5 (16-bit): page << 16 | (address << 1).
        // Channel 5 address register is at port 0xC4.
        d.write(0xC4, 0x00);
        d.write(0xC4, 0x10); // word address = 0x1000
        d.write(0x8B, 0x05); // page = 0x05
        assert_eq!(d.physical_address(5), 0x0005_2000);
    }

    #[test]
    fn status_request_bits_and_clear_on_read() {
        let mut d = dma();
        // Software-request channel 1 of controller 1.
        d.write(0x09, 0x05); // ch(1) | set(0x04)
        let status = d.read(0x08);
        // Request bit appears in the high nibble (bit 4+channel).
        assert_eq!(status & 0xF0, 0x20);
        // Inject a terminal-count bit and verify it clears on read.
        d.c1.status = 0x01;
        let s = d.read(0x08);
        assert_eq!(s & 0x0F, 0x01);
        assert_eq!(d.read(0x08) & 0x0F, 0x00);
    }

    #[test]
    fn flip_flop_independent_per_controller() {
        let mut d = dma();
        d.write(0x00, 0x11); // advances ctrl1 flip-flop only
        assert!(d.flip_flop(0));
        assert!(!d.flip_flop(1));
        d.write(0xC0, 0x22); // advances ctrl2 flip-flop only
        assert!(d.flip_flop(0));
        assert!(d.flip_flop(1));
    }
}
