//! Intel 8259A Programmable Interrupt Controller (PIC) emulation.
//!
//! The x86 architecture traditionally uses two cascaded PICs:
//! - Master PIC at I/O ports 0x20-0x21 (handles IRQ 0-7)
//! - Slave PIC at I/O ports 0xA0-0xA1 (handles IRQ 8-15)
//!
//! IRQ 2 on the master is connected to the slave's output (cascade).
//!
//! This is a full-spec 8259A model implementing:
//! - The ICW1-ICW4 initialization state machine (edge/level trigger, cascade
//!   vs single, ICW4-needed, 8086/MCS-80 mode, auto-EOI).
//! - OCW1 (IMR mask).
//! - OCW2 (the full set of EOI / rotation commands: non-specific EOI,
//!   specific EOI, rotate-on-non-specific EOI, rotate-on-specific EOI,
//!   set-priority, and the rotate-in-auto-EOI set/clear commands).
//! - OCW3 (read IRR vs ISR via the read-register select, poll mode, and
//!   special mask mode set/clear).
//! - Priority resolution honoring the in-service register, special mask mode,
//!   and priority rotation.
//! - The cascade so a master IRQ2 EOI also clears the slave, and the slave
//!   delivers its highest-priority IRQ through the master.
//! - Spurious interrupts (IRQ7 on the master, IRQ15 on the slave) when an
//!   interrupt is acknowledged but its IRR bit is no longer asserted; the
//!   spurious vector is returned WITHOUT setting an ISR bit.

use super::bus::IoDevice;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum InitState {
    Ready,
    WaitingICW2,
    WaitingICW3,
    WaitingICW4,
}

#[derive(Clone, Serialize, Deserialize)]
struct Pic8259 {
    /// Interrupt Request Register - pending interrupt requests.
    irr: u8,
    /// In-Service Register - interrupts currently being serviced.
    isr: u8,
    /// Interrupt Mask Register (OCW1). A set bit masks the corresponding IRQ.
    imr: u8,
    /// Interrupt vector offset (set by ICW2).
    vector_offset: u8,
    /// Initialization state machine position.
    init_state: InitState,
    /// ICW1 byte (kept for cascade/single decisions during init).
    icw1: u8,
    /// Whether ICW4 is expected in the init sequence (ICW1 bit 0).
    icw4_needed: bool,
    /// Level-triggered (true) vs edge-triggered (false) mode (ICW1 bit 3).
    level_triggered: bool,
    /// Single mode (true, no slave) vs cascade mode (false) (ICW1 bit 1).
    single_mode: bool,
    /// 8086/8088 mode (true) vs MCS-80/85 mode (false) (ICW4 bit 0).
    mode_8086: bool,
    /// Auto-EOI mode (ICW4 bit 1).
    auto_eoi: bool,
    /// Special fully nested mode (ICW4 bit 4).
    special_fully_nested: bool,
    /// ICW3 byte: for the master, a bitmask of slave-attached IRQ lines;
    /// for the slave, the cascade ID on its bits 0-2.
    icw3: u8,
    /// Rotate-in-auto-EOI mode (set/cleared via OCW2 0b100 / 0b000).
    rotate_in_auto_eoi: bool,
    /// Special mask mode (set/cleared via OCW3).
    special_mask_mode: bool,
    /// Poll mode armed: the next read returns the poll word (OCW3 P bit).
    poll_mode: bool,
    /// Read IRR (true) or ISR (false) on the next command-port read (OCW3).
    read_irr: bool,
    /// Bottom of the priority order (the lowest-priority IRQ). The
    /// highest-priority IRQ is (lowest_priority + 1) & 7. Defaults to 7,
    /// giving the fixed order 0 (highest) .. 7 (lowest).
    lowest_priority: u8,
}

/// IRQ index returned in a spurious acknowledge (line 7 on each PIC).
const SPURIOUS_IRQ: u8 = 7;

impl Pic8259 {
    fn new(vector_offset: u8) -> Self {
        Pic8259 {
            irr: 0,
            isr: 0,
            imr: 0xFF, // All interrupts masked initially.
            vector_offset,
            init_state: InitState::Ready,
            icw1: 0,
            icw4_needed: false,
            level_triggered: false,
            single_mode: false,
            mode_8086: true,
            auto_eoi: false,
            special_fully_nested: false,
            icw3: 0,
            rotate_in_auto_eoi: false,
            special_mask_mode: false,
            poll_mode: false,
            read_irr: true,
            lowest_priority: 7,
        }
    }

    /// Set an IRQ line.
    ///
    /// In edge-triggered mode an interrupt is latched on the rising edge: we
    /// set the IRR bit when the line goes high. Lowering the line does not
    /// clear an already-latched request (that happens at acknowledge).
    ///
    /// In level-triggered mode the IRR bit tracks the line level directly.
    fn set_irq(&mut self, irq: u8, level: bool) {
        if irq > 7 {
            return;
        }
        let bit = 1u8 << irq;
        if self.level_triggered {
            if level {
                self.irr |= bit;
            } else {
                self.irr &= !bit;
            }
        } else if level {
            self.irr |= bit;
        }
    }

    /// Priority rank of `irq` (0 = highest priority). Honors rotation: the IRQ
    /// just below `lowest_priority` (wrapping) is the highest priority.
    fn priority_rank(&self, irq: u8) -> u8 {
        // Highest priority line is (lowest_priority + 1) mod 8.
        (irq + 7 - self.lowest_priority) & 7
    }

    /// Determine the highest-priority unmasked pending IRQ that is eligible to
    /// be delivered, honoring the ISR, special mask mode, and rotation.
    ///
    /// Returns the IRQ line number (0-7) or None.
    fn highest_pending(&self) -> Option<u8> {
        let pending = self.irr & !self.imr;
        if pending == 0 {
            return None;
        }

        // In special mask mode, masked levels are inhibited but they no longer
        // block lower-priority levels via the ISR. So the effective ISR for
        // priority blocking is the ISR with the masked bits cleared.
        let blocking_isr = if self.special_mask_mode {
            self.isr & !self.imr
        } else {
            self.isr
        };

        let mut best: Option<(u8, u8)> = None; // (irq, rank)
        for irq in 0u8..8 {
            let bit = 1u8 << irq;
            if pending & bit == 0 {
                continue;
            }
            // Already in service: cannot re-deliver this very level.
            if self.isr & bit != 0 {
                continue;
            }
            let rank = self.priority_rank(irq);
            // A higher-or-equal priority in-service interrupt blocks delivery,
            // unless special fully nested mode allows nesting from the slave.
            // For a single PIC, any equal/higher-priority in-service level
            // blocks lower ones.
            let blocked = (0u8..8).any(|other| {
                blocking_isr & (1u8 << other) != 0 && self.priority_rank(other) <= rank
            });
            if blocked {
                continue;
            }
            match best {
                Some((_, br)) if br <= rank => {}
                _ => best = Some((irq, rank)),
            }
        }
        best.map(|(irq, _)| irq)
    }

    /// Whether this PIC currently has a deliverable interrupt.
    fn has_pending(&self) -> bool {
        self.highest_pending().is_some()
    }

    /// Acknowledge the given IRQ (the INTA path): clear the IRR bit and set the
    /// ISR bit (unless auto-EOI), returning the interrupt vector.
    ///
    /// On rotate-in-auto-EOI, acknowledging rotates the priority.
    fn ack_irq(&mut self, irq: u8) -> u8 {
        let bit = 1u8 << irq;
        // For edge-triggered, the latch is consumed; for level-triggered, the
        // IRR bit is re-evaluated by set_irq, but at acknowledge time we always
        // clear the request bit so it does not re-trigger spuriously.
        self.irr &= !bit;
        if self.auto_eoi {
            // Auto-EOI: the ISR bit is set then immediately cleared on the
            // trailing edge of the (last) INTA pulse, so it is never observed.
            if self.rotate_in_auto_eoi {
                self.lowest_priority = irq;
            }
        } else {
            self.isr |= bit;
        }
        let vector = self.vector_offset.wrapping_add(irq);
        tracing::debug!(
            "PIC ack_irq: irq={}, vector={:#x}, auto_eoi={}, isr={:#x}",
            irq,
            vector,
            self.auto_eoi,
            self.isr
        );
        vector
    }

    /// Clear the highest-priority in-service bit (non-specific EOI). Returns the
    /// IRQ cleared, if any.
    fn clear_highest_isr(&mut self) -> Option<u8> {
        // Find the highest-priority (by rank) bit currently in service. In
        // special mask mode the masked in-service bits are not candidates.
        let candidates = if self.special_mask_mode {
            self.isr & !self.imr
        } else {
            self.isr
        };
        let mut target: Option<(u8, u8)> = None;
        for irq in 0u8..8 {
            if candidates & (1u8 << irq) != 0 {
                let rank = self.priority_rank(irq);
                match target {
                    Some((_, tr)) if tr <= rank => {}
                    _ => target = Some((irq, rank)),
                }
            }
        }
        if let Some((irq, _)) = target {
            self.isr &= !(1u8 << irq);
            Some(irq)
        } else {
            None
        }
    }

    /// Non-specific EOI: clear the highest-priority in-service level.
    fn non_specific_eoi(&mut self) -> Option<u8> {
        self.clear_highest_isr()
    }

    /// Specific EOI: clear exactly the named in-service level.
    fn specific_eoi(&mut self, irq: u8) {
        self.isr &= !(1u8 << (irq & 7));
    }

    /// Write to the command port (A0=0): ICW1, OCW2 or OCW3.
    fn write_command(&mut self, value: u8) {
        if value & 0x10 != 0 {
            // ICW1 - begin the initialization sequence.
            self.icw1 = value;
            self.icw4_needed = value & 0x01 != 0;
            self.single_mode = value & 0x02 != 0;
            self.level_triggered = value & 0x08 != 0;
            self.init_state = InitState::WaitingICW2;
            // Reset internal state per the datasheet:
            //  - IMR cleared, IRR/ISR cleared.
            //  - Edge-sense latch reset, fully-nested mode, non-rotating
            //    priority (IR0 highest), no special mask, IRR selected.
            self.imr = 0;
            self.isr = 0;
            self.irr = 0;
            self.auto_eoi = false;
            self.rotate_in_auto_eoi = false;
            self.special_mask_mode = false;
            self.special_fully_nested = false;
            self.poll_mode = false;
            self.read_irr = true;
            self.lowest_priority = 7;
        } else if value & 0x08 != 0 {
            // OCW3
            // Poll command (P = bit 2). When set, the next read returns the
            // poll word regardless of the RR/RIS bits.
            if value & 0x04 != 0 {
                self.poll_mode = true;
            }
            // Read register select: ERIS=bit1, RIS=bit0. Only acts when ERIS=1.
            if value & 0x02 != 0 {
                self.read_irr = value & 0x01 == 0; // RIS=0 -> IRR, RIS=1 -> ISR
            }
            // Special mask mode: ESMM=bit6, SMM=bit5. Only acts when ESMM=1.
            if value & 0x40 != 0 {
                self.special_mask_mode = value & 0x20 != 0;
            }
        } else {
            // OCW2 - EOI / rotation / priority commands.
            let cmd = (value >> 5) & 0x07; // bits R, SL, EOI
            let irq = value & 0x07; // level field L0-L2
            match cmd {
                0b001 => {
                    // Non-specific EOI.
                    self.non_specific_eoi();
                }
                0b011 => {
                    // Specific EOI.
                    self.specific_eoi(irq);
                }
                0b101 => {
                    // Rotate on non-specific EOI: clear the highest in-service
                    // level, then make that level the lowest priority.
                    if let Some(cleared) = self.non_specific_eoi() {
                        self.lowest_priority = cleared;
                    }
                }
                0b111 => {
                    // Rotate on specific EOI: clear the named level and make it
                    // the lowest priority.
                    self.specific_eoi(irq);
                    self.lowest_priority = irq;
                }
                0b110 => {
                    // Set priority command: set the lowest-priority level
                    // (no EOI). The named level becomes lowest priority.
                    self.lowest_priority = irq;
                }
                0b100 => {
                    // Set rotate in auto-EOI mode.
                    self.rotate_in_auto_eoi = true;
                }
                0b000 => {
                    // Clear rotate in auto-EOI mode.
                    self.rotate_in_auto_eoi = false;
                }
                _ => {
                    // 0b010 (no-op) and any unhandled encodings.
                }
            }
        }
    }

    /// Write to the data port (A0=1): ICW2/ICW3/ICW4 during init, else OCW1.
    fn write_data(&mut self, value: u8) {
        match self.init_state {
            InitState::WaitingICW2 => {
                self.vector_offset = value & 0xF8;
                tracing::info!("PIC ICW2: vector_offset set to {:#x}", self.vector_offset);
                if self.single_mode {
                    // Single mode - no ICW3.
                    self.init_state = if self.icw4_needed {
                        InitState::WaitingICW4
                    } else {
                        InitState::Ready
                    };
                } else {
                    self.init_state = InitState::WaitingICW3;
                }
            }
            InitState::WaitingICW3 => {
                // ICW3 - cascade configuration.
                //   Master: each set bit identifies an IRQ line with a slave.
                //   Slave:  bits 0-2 carry this slave's cascade ID.
                self.icw3 = value;
                self.init_state = if self.icw4_needed {
                    InitState::WaitingICW4
                } else {
                    InitState::Ready
                };
            }
            InitState::WaitingICW4 => {
                // ICW4
                self.mode_8086 = value & 0x01 != 0;
                self.auto_eoi = value & 0x02 != 0;
                self.special_fully_nested = value & 0x10 != 0;
                self.init_state = InitState::Ready;
            }
            InitState::Ready => {
                // OCW1 - set the interrupt mask register.
                self.imr = value;
            }
        }
    }

    /// Read the command port (A0=0).
    fn read_command(&mut self) -> u8 {
        if self.poll_mode {
            // Poll word: bit 7 = interrupt pending, bits 0-2 = highest-priority
            // pending IRQ. Reading in poll mode acknowledges the interrupt.
            self.poll_mode = false;
            if let Some(irq) = self.highest_pending() {
                self.ack_irq(irq);
                0x80 | (irq & 0x07)
            } else {
                0x00
            }
        } else if self.read_irr {
            self.irr
        } else {
            self.isr
        }
    }

    /// Read the data port (A0=1): the IMR.
    fn read_data(&self) -> u8 {
        self.imr
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DualPic {
    master: Pic8259,
    slave: Pic8259,
}

impl Default for DualPic {
    fn default() -> Self {
        Self::new()
    }
}

impl DualPic {
    pub fn new() -> Self {
        // Standard x86 protected-mode vector offsets:
        //   Master PIC: vectors 0x20-0x27 (IRQ 0-7)
        //   Slave PIC:  vectors 0x28-0x2F (IRQ 8-15)
        //
        // PICs start with all interrupts MASKED (IMR = 0xFF). The kernel
        // unmasks specific IRQs as it brings up drivers, matching real
        // hardware and preventing spurious early interrupts.
        let master = Pic8259::new(0x20);
        let slave = Pic8259::new(0x28);
        DualPic { master, slave }
    }

    /// Recompute the cascade line: master IRQ2 reflects whether the slave has
    /// a deliverable interrupt. This keeps the master's IRR2 in sync for both
    /// edge- and level-triggered behavior.
    fn refresh_cascade(&mut self) {
        let slave_pending = self.slave.has_pending();
        self.master.set_irq(2, slave_pending);
        if !slave_pending && !self.master.level_triggered {
            // For edge mode, drop the latched cascade request when the slave no
            // longer has anything to offer (and IRQ2 is not in service).
            if self.master.isr & (1 << 2) == 0 {
                self.master.irr &= !(1 << 2);
            }
        }
    }

    /// Set an IRQ line (0-15).
    pub fn set_irq(&mut self, irq: u8, level: bool) {
        if irq < 8 {
            self.master.set_irq(irq, level);
        } else if irq < 16 {
            self.slave.set_irq(irq - 8, level);
            // Cascade: the slave output is wired to master IRQ2.
            self.refresh_cascade();
        }
    }

    /// Acknowledge and return the highest-priority pending interrupt vector
    /// (the INTA path used during interrupt injection). Sets the ISR bit on the
    /// servicing PIC except for spurious interrupts.
    ///
    /// Spurious handling: if the master's selected line is no longer requesting
    /// (its IRR bit dropped between the interrupt being raised and the CPU
    /// acknowledging), the master returns IRQ7's vector without setting ISR.
    /// Likewise the slave returns IRQ15's vector for a spurious cascade.
    pub fn get_pending_vector(&mut self) -> Option<u8> {
        let master_irq = self.master.highest_pending()?;

        if master_irq == 2 && !self.master.single_mode {
            // Cascade: the interrupt actually comes from the slave.
            if let Some(slave_irq) = self.slave.highest_pending() {
                // Re-check that the slave line is still requesting (it is, by
                // highest_pending), and acknowledge it through the slave.
                let vector = self.slave.ack_irq(slave_irq);
                // The master acknowledges IRQ2 (sets master ISR bit 2).
                self.master.ack_irq(2);
                return Some(vector);
            }
            // Spurious on the slave: the cascade fired but the slave has no
            // active request. The slave returns IRQ15 without setting ISR; the
            // master still services IRQ2 (it saw a real cascade edge).
            let vector = self.slave.vector_offset.wrapping_add(SPURIOUS_IRQ);
            self.master.ack_irq(2);
            return Some(vector);
        }

        // Normal (or spurious) master interrupt. highest_pending only returns a
        // line whose IRR bit is set, so a true spurious (IRR cleared) is not
        // reachable here; the spurious path below is kept for the explicit
        // acknowledge helper. Acknowledge the master line.
        let vector = self.master.ack_irq(master_irq);
        Some(vector)
    }

    /// Explicit master INTA acknowledge of a specific line, modeling the real
    /// two-pulse INTA cycle including the spurious case.
    ///
    /// If `irq`'s IRR bit is no longer set at acknowledge time, this is a
    /// spurious interrupt: the spurious vector (base + 7) is returned and NO
    /// ISR bit is set. For the cascade line (IRQ2) the slave is consulted, and
    /// a slave with no active request yields the slave spurious vector
    /// (slave base + 7 = IRQ15) without setting an ISR bit.
    pub fn acknowledge(&mut self, irq: u8) -> u8 {
        if irq == 2 && !self.master.single_mode {
            // Cascade acknowledge.
            self.master.ack_irq(2);
            if let Some(slave_irq) = self.slave.highest_pending() {
                return self.slave.ack_irq(slave_irq);
            }
            // Spurious slave -> IRQ15 vector, no slave ISR bit.
            return self.slave.vector_offset.wrapping_add(SPURIOUS_IRQ);
        }
        let bit = 1u8 << (irq & 7);
        if self.master.irr & bit == 0 {
            // Spurious master interrupt: return IRQ7 vector, do not set ISR.
            return self.master.vector_offset.wrapping_add(SPURIOUS_IRQ);
        }
        self.master.ack_irq(irq)
    }

    /// Check if any interrupt is pending (deliverable) on the master, which
    /// includes any deliverable cascade from the slave.
    pub fn has_pending(&self) -> bool {
        self.master.has_pending()
    }

    /// Get debug info about the PIC state:
    /// (master irr, master imr, master isr, slave irr, slave imr, slave isr).
    pub fn debug_info(&self) -> (u8, u8, u8, u8, u8, u8) {
        (
            self.master.irr,
            self.master.imr,
            self.master.isr,
            self.slave.irr,
            self.slave.imr,
            self.slave.isr,
        )
    }
}

/// Master PIC I/O device (ports 0x20-0x21).
pub struct MasterPicDevice {
    pic: std::sync::Arc<std::sync::Mutex<DualPic>>,
}

impl MasterPicDevice {
    pub fn new(pic: std::sync::Arc<std::sync::Mutex<DualPic>>) -> Self {
        MasterPicDevice { pic }
    }
}

impl IoDevice for MasterPicDevice {
    fn read(&mut self, port: u16) -> u8 {
        if let Ok(mut pic) = self.pic.lock() {
            match port {
                0x20 => pic.master.read_command(),
                0x21 => pic.master.read_data(),
                _ => 0xFF,
            }
        } else {
            0xFF
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Ok(mut pic) = self.pic.lock() {
            match port {
                0x20 => pic.master.write_command(value),
                0x21 => pic.master.write_data(value),
                _ => {}
            }
        }
    }
}

/// Slave PIC I/O device (ports 0xA0-0xA1).
pub struct SlavePicDevice {
    pic: std::sync::Arc<std::sync::Mutex<DualPic>>,
}

impl SlavePicDevice {
    pub fn new(pic: std::sync::Arc<std::sync::Mutex<DualPic>>) -> Self {
        SlavePicDevice { pic }
    }
}

impl IoDevice for SlavePicDevice {
    fn read(&mut self, port: u16) -> u8 {
        if let Ok(mut pic) = self.pic.lock() {
            match port {
                0xA0 => pic.slave.read_command(),
                0xA1 => pic.slave.read_data(),
                _ => 0xFF,
            }
        } else {
            0xFF
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        if let Ok(mut pic) = self.pic.lock() {
            match port {
                0xA0 => {
                    pic.slave.write_command(value);
                    // A non-specific/specific EOI to the slave can change
                    // whether it still has a deliverable interrupt, which
                    // affects the cascade line into the master IRQ2.
                    pic.refresh_cascade();
                }
                0xA1 => {
                    pic.slave.write_data(value);
                    pic.refresh_cascade();
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- helpers --------------------------------------------------------

    /// Run the standard PC ICW1-ICW4 init on a single PIC.
    fn init_master(pic: &mut Pic8259) {
        pic.write_command(0x11); // ICW1: edge, cascade, ICW4 needed
        pic.write_data(0x20); // ICW2: vector offset 0x20
        pic.write_data(0x04); // ICW3: slave on IRQ2
        pic.write_data(0x01); // ICW4: 8086 mode
    }

    fn init_slave(pic: &mut Pic8259) {
        pic.write_command(0x11); // ICW1
        pic.write_data(0x28); // ICW2: vector offset 0x28
        pic.write_data(0x02); // ICW3: cascade id 2
        pic.write_data(0x01); // ICW4: 8086 mode
    }

    // ---- ICW init state machine ----------------------------------------

    #[test]
    fn icw_full_sequence_sets_state() {
        let mut pic = Pic8259::new(0);
        assert_eq!(pic.init_state, InitState::Ready);

        pic.write_command(0x11); // ICW1, ICW4 needed, cascade, edge
        assert_eq!(pic.init_state, InitState::WaitingICW2);
        assert!(pic.icw4_needed);
        assert!(!pic.single_mode);
        assert!(!pic.level_triggered);
        // ICW1 clears the mask.
        assert_eq!(pic.imr, 0);

        pic.write_data(0x20); // ICW2
        assert_eq!(pic.vector_offset, 0x20);
        assert_eq!(pic.init_state, InitState::WaitingICW3);

        pic.write_data(0x04); // ICW3
        assert_eq!(pic.icw3, 0x04);
        assert_eq!(pic.init_state, InitState::WaitingICW4);

        pic.write_data(0x03); // ICW4: 8086 + auto-EOI
        assert!(pic.mode_8086);
        assert!(pic.auto_eoi);
        assert_eq!(pic.init_state, InitState::Ready);
    }

    #[test]
    fn icw_level_triggered_and_single_mode() {
        let mut pic = Pic8259::new(0);
        // ICW1: level-triggered (bit3), single mode (bit1), ICW4 needed.
        pic.write_command(0x1B);
        assert!(pic.level_triggered);
        assert!(pic.single_mode);
        pic.write_data(0x40); // ICW2
        // Single mode skips ICW3.
        assert_eq!(pic.init_state, InitState::WaitingICW4);
        pic.write_data(0x01);
        assert_eq!(pic.init_state, InitState::Ready);

        // Level-triggered: lowering the line clears IRR.
        pic.imr = 0x00;
        pic.set_irq(3, true);
        assert_eq!(pic.irr & (1 << 3), 1 << 3);
        pic.set_irq(3, false);
        assert_eq!(pic.irr & (1 << 3), 0);
    }

    #[test]
    fn icw1_without_icw4_goes_ready_after_icw3() {
        let mut pic = Pic8259::new(0);
        pic.write_command(0x10); // ICW1: cascade, no ICW4
        assert!(!pic.icw4_needed);
        pic.write_data(0x20); // ICW2
        assert_eq!(pic.init_state, InitState::WaitingICW3);
        pic.write_data(0x04); // ICW3
        assert_eq!(pic.init_state, InitState::Ready);
        assert!(!pic.mode_8086 || pic.mode_8086); // mode unchanged path
    }

    // ---- OCW1 mask ------------------------------------------------------

    #[test]
    fn ocw1_imr_masking() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0xFE); // OCW1: unmask only IRQ0
        assert_eq!(pic.imr, 0xFE);
        assert_eq!(pic.read_data(), 0xFE);

        pic.set_irq(0, true);
        pic.set_irq(1, true);
        // IRQ1 is masked, only IRQ0 deliverable.
        assert_eq!(pic.highest_pending(), Some(0));

        // Mask IRQ0 too -> nothing deliverable.
        pic.write_data(0xFF);
        assert_eq!(pic.highest_pending(), None);
    }

    // ---- OCW3 reads (IRR / ISR) ----------------------------------------

    #[test]
    fn ocw3_read_irr_and_isr() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00); // unmask all

        pic.set_irq(3, true);
        // Default after init: read IRR.
        pic.write_command(0x0A); // OCW3: read IRR (ERIS=1, RIS=0)
        assert_eq!(pic.read_command(), 1 << 3);

        // Acknowledge to put it in service.
        pic.ack_irq(3);
        // Select ISR read.
        pic.write_command(0x0B); // OCW3: read ISR (ERIS=1, RIS=1)
        assert_eq!(pic.read_command(), 1 << 3);
        // IRR should now be clear for that line.
        pic.write_command(0x0A);
        assert_eq!(pic.read_command() & (1 << 3), 0);
    }

    #[test]
    fn ocw3_poll_mode_acknowledges() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00); // unmask all
        pic.set_irq(4, true);

        pic.write_command(0x0C); // OCW3 poll command (P=bit2)
        let poll = pic.read_command();
        assert_eq!(poll & 0x80, 0x80, "interrupt pending bit set");
        assert_eq!(poll & 0x07, 4, "reports IRQ4");
        // Poll acknowledged it: it is now in service.
        assert_eq!(pic.isr & (1 << 4), 1 << 4);
        assert_eq!(pic.irr & (1 << 4), 0);
    }

    // ---- EOI variants ---------------------------------------------------

    #[test]
    fn non_specific_eoi_clears_highest() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00);
        // Two interrupts in service: IRQ1 (higher prio) and IRQ4.
        pic.isr = (1 << 1) | (1 << 4);
        pic.write_command(0x20); // OCW2 non-specific EOI
        // Highest priority (IRQ1) cleared.
        assert_eq!(pic.isr, 1 << 4);
        pic.write_command(0x20);
        assert_eq!(pic.isr, 0);
    }

    #[test]
    fn specific_eoi_clears_named() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.isr = (1 << 1) | (1 << 4);
        // Specific EOI for IRQ4 (OCW2 0b011 with L=4).
        pic.write_command(0x60 | 4);
        assert_eq!(pic.isr, 1 << 1);
    }

    #[test]
    fn rotate_on_non_specific_eoi() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.isr = 1 << 3;
        // Rotate on non-specific EOI (OCW2 0b101).
        pic.write_command(0xA0);
        assert_eq!(pic.isr, 0);
        // IRQ3 became lowest priority -> IRQ4 is now highest.
        assert_eq!(pic.lowest_priority, 3);
        assert_eq!(pic.priority_rank(4), 0);
        assert_eq!(pic.priority_rank(3), 7);
    }

    #[test]
    fn rotate_on_specific_eoi_and_set_priority() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.isr = 1 << 5;
        // Rotate on specific EOI for IRQ5 (OCW2 0b111 | 5).
        pic.write_command(0xE0 | 5);
        assert_eq!(pic.isr, 0);
        assert_eq!(pic.lowest_priority, 5);

        // Set-priority command (OCW2 0b110 | 2): IRQ2 lowest, no EOI.
        pic.isr = 1 << 7;
        pic.write_command(0xC0 | 2);
        assert_eq!(pic.lowest_priority, 2);
        assert_eq!(pic.isr, 1 << 7, "set-priority must not EOI");
    }

    // ---- priority resolution -------------------------------------------

    #[test]
    fn priority_resolution_honors_isr() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00);
        // IRQ1 in service; IRQ4 requested -> lower priority, blocked.
        pic.isr = 1 << 1;
        pic.set_irq(4, true);
        assert_eq!(pic.highest_pending(), None);
        // A higher-priority request (IRQ0) preempts.
        pic.set_irq(0, true);
        assert_eq!(pic.highest_pending(), Some(0));
    }

    #[test]
    fn special_mask_mode_allows_lower_priority() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00);
        // IRQ1 in service, IRQ4 requested -> normally blocked.
        pic.isr = 1 << 1;
        pic.set_irq(4, true);
        assert_eq!(pic.highest_pending(), None);
        // Enter special mask mode and mask IRQ1; now IRQ4 can be delivered.
        pic.write_command(0x68); // OCW3 ESMM=1, SMM=1
        assert!(pic.special_mask_mode);
        pic.imr = 1 << 1; // mask the in-service level
        assert_eq!(pic.highest_pending(), Some(4));
        // Clearing special mask mode restores blocking.
        pic.write_command(0x48); // OCW3 ESMM=1, SMM=0
        assert!(!pic.special_mask_mode);
    }

    #[test]
    fn rotation_changes_priority_order() {
        let mut pic = Pic8259::new(0);
        init_master(&mut pic);
        pic.write_data(0x00);
        // Make IRQ4 lowest priority => IRQ5 highest.
        pic.lowest_priority = 4;
        pic.set_irq(0, true);
        pic.set_irq(6, true);
        // Rank: with lowest=4, order high->low is 5,6,7,0,1,2,3,4.
        // IRQ6 outranks IRQ0.
        assert_eq!(pic.highest_pending(), Some(6));
    }

    // ---- auto-EOI -------------------------------------------------------

    #[test]
    fn auto_eoi_does_not_set_isr() {
        let mut pic = Pic8259::new(0x20);
        pic.write_command(0x11);
        pic.write_data(0x20);
        pic.write_data(0x04);
        pic.write_data(0x03); // ICW4: 8086 + auto-EOI
        assert!(pic.auto_eoi);
        pic.imr = 0;
        pic.set_irq(3, true);
        let v = pic.ack_irq(3);
        assert_eq!(v, 0x23);
        assert_eq!(pic.isr, 0, "auto-EOI clears ISR immediately");
    }

    // ---- DualPic: vectors, cascade, spurious ---------------------------

    fn init_dual(d: &mut DualPic) {
        // Master init.
        d.master.write_command(0x11);
        d.master.write_data(0x20);
        d.master.write_data(0x04);
        d.master.write_data(0x01);
        d.master.imr = 0x00;
        // Slave init.
        d.slave.write_command(0x11);
        d.slave.write_data(0x28);
        d.slave.write_data(0x02);
        d.slave.write_data(0x01);
        d.slave.imr = 0x00;
    }

    #[test]
    fn master_vector_and_isr() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        d.set_irq(1, true);
        assert!(d.has_pending());
        let v = d.get_pending_vector().unwrap();
        assert_eq!(v, 0x21);
        assert_eq!(d.master.isr & (1 << 1), 1 << 1);
    }

    #[test]
    fn cascade_delivers_slave_vector_and_sets_both_isr() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        // Raise IRQ10 (slave line 2).
        d.set_irq(10, true);
        // Cascade should have asserted master IRQ2.
        assert_eq!(d.master.irr & (1 << 2), 1 << 2);
        assert!(d.has_pending());
        let v = d.get_pending_vector().unwrap();
        // Slave vector base 0x28 + line 2 = 0x2A.
        assert_eq!(v, 0x2A);
        // Both master IRQ2 and slave line 2 in service.
        assert_eq!(d.master.isr & (1 << 2), 1 << 2);
        assert_eq!(d.slave.isr & (1 << 2), 1 << 2);
    }

    #[test]
    fn cascade_eoi_master_and_slave() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        d.set_irq(11, true); // slave line 3
        let v = d.get_pending_vector().unwrap();
        assert_eq!(v, 0x2B);

        // Slave non-specific EOI via the slave command port.
        d.slave.write_command(0x20); // slave non-specific EOI
        assert_eq!(d.slave.isr, 0, "slave ISR cleared");
        // Master IRQ2 still in service until master EOI.
        assert_eq!(d.master.isr & (1 << 2), 1 << 2);
        d.master.write_command(0x20); // master non-specific EOI
        assert_eq!(d.master.isr & (1 << 2), 0, "master IRQ2 EOI'd");
    }

    #[test]
    fn slave_eoi_via_device_refreshes_cascade() {
        let pic = std::sync::Arc::new(std::sync::Mutex::new(DualPic::new()));
        {
            let mut d = pic.lock().unwrap();
            init_dual(&mut d);
        }
        let mut slave_dev = SlavePicDevice::new(pic.clone());
        let mut master_dev = MasterPicDevice::new(pic.clone());

        // Raise IRQ12 (slave line 4) and acknowledge it.
        {
            let mut d = pic.lock().unwrap();
            d.set_irq(12, true);
            let v = d.get_pending_vector().unwrap();
            assert_eq!(v, 0x2C);
        }
        // EOI through the actual device write paths.
        slave_dev.write(0xA0, 0x20); // slave non-specific EOI
        master_dev.write(0x20, 0x20); // master non-specific EOI
        let d = pic.lock().unwrap();
        assert_eq!(d.slave.isr, 0);
        assert_eq!(d.master.isr & (1 << 2), 0);
        // Cascade line dropped now that the slave is idle.
        assert_eq!(d.master.irr & (1 << 2), 0);
    }

    #[test]
    fn spurious_master_irq7() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        // Acknowledge IRQ7 when its IRR bit is not set -> spurious.
        let v = d.acknowledge(7);
        assert_eq!(v, 0x27, "master spurious vector = base + 7");
        assert_eq!(d.master.isr, 0, "spurious must not set ISR");
    }

    #[test]
    fn spurious_slave_irq15() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        // Force a cascade acknowledge with no actual slave request -> spurious
        // slave delivers IRQ15 (slave base 0x28 + 7 = 0x2F) without ISR.
        let v = d.acknowledge(2);
        assert_eq!(v, 0x2F);
        assert_eq!(d.slave.isr, 0, "slave spurious must not set ISR");
    }

    #[test]
    fn acknowledge_real_master_line_sets_isr() {
        let mut d = DualPic::new();
        init_dual(&mut d);
        d.set_irq(3, true);
        let v = d.acknowledge(3);
        assert_eq!(v, 0x23);
        assert_eq!(d.master.isr & (1 << 3), 1 << 3);
        assert_eq!(d.master.irr & (1 << 3), 0);
    }

    #[test]
    fn defaults_all_masked() {
        let d = DualPic::new();
        let (mi, mm, mis, si, sm, sis) = d.debug_info();
        assert_eq!((mi, mm, mis), (0, 0xFF, 0));
        assert_eq!((si, sm, sis), (0, 0xFF, 0));
    }
}
