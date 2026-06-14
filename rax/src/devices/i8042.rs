//! Intel 8042 PS/2 Keyboard/Mouse Controller emulation.
//!
//! The 8042 is the "keyboard controller" found on every PC-compatible
//! system. It exposes two I/O ports:
//!
//! - `0x60`: Data port (read = output buffer, write = input buffer / data)
//! - `0x64`: Status register (read) / command register (write)
//!
//! Two devices hang off the controller:
//! - The first PS/2 port (keyboard), whose buffer-full events raise IRQ 1.
//! - The second PS/2 port (auxiliary / mouse), which raises IRQ 12.
//!
//! # Interrupt model
//!
//! Like the other devices in this crate (see [`crate::devices::pit`] and
//! [`crate::devices::serial`]), the 8042 does not own an IRQ callback. Instead
//! it exposes pollable "interrupt pending" flags that the VMM samples each loop
//! iteration and forwards to the PIC. Keyboard data raises IRQ 1 (when command
//! byte bit 0 is set) and auxiliary data raises IRQ 12 (when command byte bit 1
//! is set). The orchestrator queries [`I8042::keyboard_irq_pending`] /
//! [`I8042::aux_irq_pending`] and clears them with the matching `clear_*` calls.

use std::collections::VecDeque;

use super::bus::IoDevice;

/// Data port (output buffer on read, input buffer on write).
pub const DATA_PORT: u16 = 0x60;
/// Status register (read) / command register (write).
pub const STATUS_PORT: u16 = 0x64;

// ---------------------------------------------------------------------------
// Status register bits (read from 0x64)
// ---------------------------------------------------------------------------

/// Output buffer full: data is available to read from 0x60.
const STATUS_OBF: u8 = 1 << 0;
/// Input buffer full: a byte written to 0x60/0x64 has not yet been consumed.
const STATUS_IBF: u8 = 1 << 1;
/// System flag: set after a successful self-test (POST passed).
const STATUS_SYSF: u8 = 1 << 2;
/// Command/data (A2): set when the byte in the input buffer was a command
/// (written to 0x64), clear when it was data (written to 0x60).
const STATUS_A2: u8 = 1 << 3;
/// Keyboard inhibited (keyboard lock). Kept high here (unlocked) per typical
/// BIOS expectations.
const STATUS_INH: u8 = 1 << 4;
/// Auxiliary output buffer full: the byte in the output buffer came from the
/// auxiliary (mouse) device rather than the keyboard. (a.k.a. MOBF / AUXB)
const STATUS_AUX: u8 = 1 << 5;
/// Timeout error.
const STATUS_TIMEOUT: u8 = 1 << 6;
/// Parity error.
const STATUS_PARITY: u8 = 1 << 7;

// ---------------------------------------------------------------------------
// Command byte bits (the "controller configuration byte", index 0)
// ---------------------------------------------------------------------------

/// Enable IRQ 1 on keyboard output-buffer-full.
const CMD_BYTE_KBD_IRQ: u8 = 1 << 0;
/// Enable IRQ 12 on auxiliary output-buffer-full.
const CMD_BYTE_AUX_IRQ: u8 = 1 << 1;
/// System flag (mirrored into the status register's SYSF bit).
const CMD_BYTE_SYSF: u8 = 1 << 2;
/// Disable keyboard clock.
const CMD_BYTE_KBD_DISABLE: u8 = 1 << 4;
/// Disable auxiliary clock.
const CMD_BYTE_AUX_DISABLE: u8 = 1 << 5;
/// Keyboard scancode translation enabled.
const CMD_BYTE_TRANSLATE: u8 = 1 << 6;

// ---------------------------------------------------------------------------
// Controller commands (written to 0x64)
// ---------------------------------------------------------------------------

const CMD_READ_CMD_BYTE: u8 = 0x20;
const CMD_WRITE_CMD_BYTE: u8 = 0x60;
const CMD_DISABLE_AUX: u8 = 0xA7;
const CMD_ENABLE_AUX: u8 = 0xA8;
const CMD_TEST_AUX: u8 = 0xA9;
const CMD_SELF_TEST: u8 = 0xAA;
const CMD_TEST_KBD: u8 = 0xAB;
const CMD_DISABLE_KBD: u8 = 0xAD;
const CMD_ENABLE_KBD: u8 = 0xAE;
const CMD_READ_OUTPUT_PORT: u8 = 0xD0;
const CMD_WRITE_OUTPUT_PORT: u8 = 0xD1;
const CMD_WRITE_AUX: u8 = 0xD4;
// 0xF0..=0xFF pulse the output line; the low nibble is a mask of lines to
// pulse low. Bit 0 (line P0) is the CPU reset line, so 0xFE pulses reset.
const CMD_PULSE_BASE: u8 = 0xF0;

/// Self-test reply byte ("controller OK").
const SELF_TEST_OK: u8 = 0x55;
/// Interface (port) test reply byte ("no error").
const PORT_TEST_OK: u8 = 0x00;

// ---------------------------------------------------------------------------
// PS/2 mouse (auxiliary device) command set, sent via 0x64=0xD4 then 0x60.
// ---------------------------------------------------------------------------

/// Acknowledge byte the mouse returns for (almost) every command.
const MOUSE_ACK: u8 = 0xFA;
/// Self-test-passed byte (BAT completion) returned after a reset.
const MOUSE_SELF_TEST_OK: u8 = 0xAA;

const MOUSE_CMD_RESET: u8 = 0xFF;
const MOUSE_CMD_RESEND: u8 = 0xFE;
const MOUSE_CMD_SET_DEFAULTS: u8 = 0xF6;
const MOUSE_CMD_DISABLE_REPORTING: u8 = 0xF5;
const MOUSE_CMD_ENABLE_REPORTING: u8 = 0xF4;
const MOUSE_CMD_SET_SAMPLE_RATE: u8 = 0xF3;
const MOUSE_CMD_GET_DEVICE_ID: u8 = 0xF2;
const MOUSE_CMD_SET_REMOTE_MODE: u8 = 0xF0;
const MOUSE_CMD_READ_DATA: u8 = 0xEB;
const MOUSE_CMD_SET_STREAM_MODE: u8 = 0xEA;
const MOUSE_CMD_STATUS_REQUEST: u8 = 0xE9;
const MOUSE_CMD_SET_RESOLUTION: u8 = 0xE8;
const MOUSE_CMD_SET_SCALING_2_1: u8 = 0xE7;
const MOUSE_CMD_SET_SCALING_1_1: u8 = 0xE6;

/// Standard 2-button PS/2 mouse device id.
const MOUSE_ID_STANDARD: u8 = 0x00;
/// IntelliMouse (scroll wheel) device id, unlocked by the 200/100/80 sequence.
const MOUSE_ID_INTELLIMOUSE: u8 = 0x03;

/// Movement-packet byte 0: bit 3 is always 1 on a PS/2 mouse.
const MOUSE_PKT_ALWAYS_ONE: u8 = 1 << 3;
/// Movement-packet byte 0: left button currently down.
const MOUSE_PKT_LEFT_BTN: u8 = 1 << 0;
/// Movement-packet byte 0: right button currently down.
const MOUSE_PKT_RIGHT_BTN: u8 = 1 << 1;
/// Movement-packet byte 0: middle button currently down.
const MOUSE_PKT_MIDDLE_BTN: u8 = 1 << 2;
/// Movement-packet byte 0: X movement is negative (sign bit).
const MOUSE_PKT_X_SIGN: u8 = 1 << 4;
/// Movement-packet byte 0: Y movement is negative (sign bit).
const MOUSE_PKT_Y_SIGN: u8 = 1 << 5;
/// Movement-packet byte 0: X movement overflowed.
const MOUSE_PKT_X_OVERFLOW: u8 = 1 << 6;
/// Movement-packet byte 0: Y movement overflowed.
const MOUSE_PKT_Y_OVERFLOW: u8 = 1 << 7;

/// Whether the auxiliary device awaits a parameter byte for a mouse command.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MousePending {
    /// No parameter outstanding.
    None,
    /// Next aux data byte is the sample-rate parameter (after 0xF3).
    SampleRate,
    /// Next aux data byte is the resolution parameter (after 0xE8).
    Resolution,
}

/// State of the emulated PS/2 mouse hanging off the auxiliary port.
struct MouseState {
    /// True when the device streams movement packets (0xF4 / off via 0xF5).
    reporting: bool,
    /// True for stream mode (default), false for remote (polled) mode.
    stream_mode: bool,
    /// Reported sample rate in samples/second (0xF3).
    sample_rate: u8,
    /// Reported resolution code 0..=3 (counts/mm: 1, 2, 4, 8) (0xE8).
    resolution: u8,
    /// True when 2:1 scaling is active (0xE7), false for 1:1 (0xE6).
    scaling_2_1: bool,
    /// Current device id: 0x00 standard, 0x03 IntelliMouse.
    device_id: u8,
    /// Awaited parameter byte, if any.
    pending: MousePending,
    /// Rolling record of the last three sample-rate values for the
    /// IntelliMouse 200/100/80 magic-knock detection.
    rate_history: [u8; 3],
    /// Last button state, replayed by 0xEB / remote-mode reads.
    buttons: u8,
}

impl MouseState {
    fn new() -> Self {
        let mut state = MouseState {
            reporting: false,
            stream_mode: true,
            sample_rate: 100,
            resolution: 2,
            scaling_2_1: false,
            device_id: MOUSE_ID_STANDARD,
            pending: MousePending::None,
            rate_history: [0; 3],
            buttons: 0,
        };
        state.apply_defaults();
        state
    }

    /// Reset all configurable parameters to power-on defaults (used by both
    /// 0xFF reset and 0xF6 set-defaults). Does not change the device id, which
    /// the caller manages explicitly.
    fn apply_defaults(&mut self) {
        self.reporting = false;
        self.stream_mode = true;
        self.sample_rate = 100;
        self.resolution = 2;
        self.scaling_2_1 = false;
        self.pending = MousePending::None;
    }

    /// True when the device is operating as an IntelliMouse (4-byte packets).
    fn is_intellimouse(&self) -> bool {
        self.device_id == MOUSE_ID_INTELLIMOUSE
    }
}

/// Output port reset default. Bit 0 (system reset) is held high (inactive),
/// bit 1 (A20 gate) is held high (A20 enabled), matching a post-BIOS machine.
const OUTPUT_PORT_RESET: u8 = 0b0000_0011;
/// Output-port bit 0: system reset line. Active-low; pulsing it low resets.
const OUTPUT_PORT_RESET_LINE: u8 = 1 << 0;
/// Output-port bit 1: A20 gate. When set, A20 is enabled.
const OUTPUT_PORT_A20: u8 = 1 << 1;

/// Pending command that expects a following data byte on the next 0x60 write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PendingWrite {
    None,
    /// Next 0x60 write sets the controller command byte (after 0x60 command).
    CommandByte,
    /// Next 0x60 write sets the output port (after 0xD1 command).
    OutputPort,
    /// Next 0x60 write is forwarded to the auxiliary device (after 0xD4).
    AuxByte,
}

/// Intel 8042 PS/2 controller.
pub struct I8042 {
    /// Controller configuration ("command") byte.
    command_byte: u8,
    /// Status register bits not derived from other state (TIMEOUT/PARITY/INH).
    status_extra: u8,
    /// Whether the controller passed self-test (drives SYSF).
    self_test_passed: bool,
    /// The output port latch (A20 gate, reset line, etc.).
    output_port: u8,

    /// Keyboard scancode queue feeding the output buffer.
    kbd_queue: VecDeque<u8>,
    /// Auxiliary (mouse) byte queue feeding the output buffer.
    aux_queue: VecDeque<u8>,

    /// Latched byte currently presented on the output buffer (port 0x60 read).
    output_byte: Option<u8>,
    /// True when the latched output byte came from the auxiliary device.
    output_is_aux: bool,

    /// Pending command awaiting its data byte.
    pending: PendingWrite,

    /// Whether the keyboard port is enabled.
    kbd_enabled: bool,
    /// Whether the auxiliary port is enabled.
    aux_enabled: bool,

    /// Sticky "the guest requested a CPU reset via 0xFE / output-port bit 0".
    reset_requested: bool,

    /// Pollable IRQ-pending flags (sampled and cleared by the VMM).
    kbd_irq_pending: bool,
    aux_irq_pending: bool,

    /// State of the PS/2 mouse hanging off the auxiliary port.
    mouse: MouseState,
}

impl Default for I8042 {
    fn default() -> Self {
        Self::new()
    }
}

impl I8042 {
    /// Create a freshly-reset controller.
    ///
    /// Mirrors typical power-on state: both ports enabled, keyboard IRQ
    /// enabled, self-test not yet run, A20 gate on.
    pub fn new() -> Self {
        I8042 {
            // Keyboard IRQ on, translation on, both clocks enabled.
            command_byte: CMD_BYTE_KBD_IRQ | CMD_BYTE_TRANSLATE,
            status_extra: STATUS_INH,
            self_test_passed: false,
            output_port: OUTPUT_PORT_RESET,
            kbd_queue: VecDeque::new(),
            aux_queue: VecDeque::new(),
            output_byte: None,
            output_is_aux: false,
            pending: PendingWrite::None,
            kbd_enabled: true,
            aux_enabled: true,
            reset_requested: false,
            kbd_irq_pending: false,
            aux_irq_pending: false,
            mouse: MouseState::new(),
        }
    }

    // ----- Public input plumbing -------------------------------------------

    /// Push a keyboard scancode toward the guest.
    ///
    /// The byte enters the keyboard FIFO; if the output buffer is free it is
    /// promoted immediately, setting OBF and (if enabled) raising IRQ 1.
    pub fn push_keyboard_scancode(&mut self, scancode: u8) {
        if !self.kbd_enabled {
            return;
        }
        self.kbd_queue.push_back(scancode);
        self.refill_output_buffer();
    }

    /// Push an auxiliary (PS/2 mouse) byte toward the guest.
    ///
    /// The byte enters the auxiliary FIFO; if the output buffer is free it is
    /// promoted immediately, setting OBF+AUX and (if enabled) raising IRQ 12.
    pub fn push_mouse_byte(&mut self, byte: u8) {
        if !self.aux_enabled {
            return;
        }
        self.aux_queue.push_back(byte);
        self.refill_output_buffer();
    }

    /// Report mouse movement / button state from the host toward the guest.
    ///
    /// When data reporting is enabled this queues a standard PS/2 movement
    /// packet on the auxiliary stream (3 bytes, or 4 bytes including the
    /// scroll-wheel delta when operating as an IntelliMouse), raising IRQ 12 if
    /// the controller has the auxiliary interrupt enabled.
    ///
    /// `dx`/`dy` are relative motion deltas (positive `dy` is "up", matching the
    /// PS/2 convention where the Y axis points up). `buttons` carries the
    /// current button state in bits 0/1/2 (left/right/middle). `dz` is the
    /// scroll-wheel delta, only emitted for an IntelliMouse.
    pub fn push_mouse_movement(&mut self, dx: i16, dy: i16, buttons: u8, dz: i8) {
        // Track button state so 0xEB / remote reads can replay it.
        self.mouse.buttons = buttons & 0b0000_0111;

        // Stream-mode reporting must be enabled for spontaneous packets.
        if !self.mouse.reporting || !self.mouse.stream_mode {
            return;
        }

        self.queue_movement_packet(dx, dy, buttons, dz);
    }

    /// Build and enqueue a movement packet onto the aux stream.
    fn queue_movement_packet(&mut self, dx: i16, dy: i16, buttons: u8, dz: i8) {
        // Clamp the 9-bit-signed deltas, recording overflow per the spec.
        let (x, x_over) = clamp_delta(dx);
        let (y, y_over) = clamp_delta(dy);

        let mut flags = MOUSE_PKT_ALWAYS_ONE;
        if (buttons & MOUSE_PKT_LEFT_BTN) != 0 {
            flags |= MOUSE_PKT_LEFT_BTN;
        }
        if (buttons & MOUSE_PKT_RIGHT_BTN) != 0 {
            flags |= MOUSE_PKT_RIGHT_BTN;
        }
        if (buttons & MOUSE_PKT_MIDDLE_BTN) != 0 {
            flags |= MOUSE_PKT_MIDDLE_BTN;
        }
        if x < 0 {
            flags |= MOUSE_PKT_X_SIGN;
        }
        if y < 0 {
            flags |= MOUSE_PKT_Y_SIGN;
        }
        if x_over {
            flags |= MOUSE_PKT_X_OVERFLOW;
        }
        if y_over {
            flags |= MOUSE_PKT_Y_OVERFLOW;
        }

        self.push_mouse_byte(flags);
        self.push_mouse_byte((x & 0xFF) as u8);
        self.push_mouse_byte((y & 0xFF) as u8);
        if self.mouse.is_intellimouse() {
            // Byte 3 is the signed Z (scroll) delta, clamped to 4 bits.
            let z = dz.clamp(-8, 7);
            self.push_mouse_byte((z as u8) & 0x0F);
        }
    }

    // ----- Queryable state for the orchestrator ----------------------------

    /// True if IRQ 1 (keyboard) is asserted and pending injection.
    pub fn keyboard_irq_pending(&self) -> bool {
        self.kbd_irq_pending
    }

    /// True if IRQ 12 (auxiliary/mouse) is asserted and pending injection.
    pub fn aux_irq_pending(&self) -> bool {
        self.aux_irq_pending
    }

    /// Clear the latched keyboard IRQ-pending flag (after it is injected).
    pub fn clear_keyboard_irq(&mut self) {
        self.kbd_irq_pending = false;
    }

    /// Clear the latched auxiliary IRQ-pending flag (after it is injected).
    pub fn clear_aux_irq(&mut self) {
        self.aux_irq_pending = false;
    }

    /// True if the guest asked the controller to reset the CPU (0xFE pulse or
    /// driving output-port bit 0 low). Sticky until [`Self::clear_reset_request`].
    pub fn reset_requested(&self) -> bool {
        self.reset_requested
    }

    /// Acknowledge / clear the pending CPU-reset request.
    pub fn clear_reset_request(&mut self) {
        self.reset_requested = false;
    }

    /// Current A20 gate state (true = A20 enabled / unmasked).
    pub fn a20_enabled(&self) -> bool {
        (self.output_port & OUTPUT_PORT_A20) != 0
    }

    /// Raw output-port latch value (A20 gate, reset line, etc.).
    pub fn output_port(&self) -> u8 {
        self.output_port
    }

    // ----- Internal helpers ------------------------------------------------

    /// Compute the status register value presented on a read of 0x64.
    fn status(&self) -> u8 {
        let mut status = self.status_extra & (STATUS_INH | STATUS_TIMEOUT | STATUS_PARITY);

        if self.output_byte.is_some() {
            status |= STATUS_OBF;
            if self.output_is_aux {
                status |= STATUS_AUX;
            }
        }
        // STATUS_IBF (input buffer full) is intentionally never asserted here.
        // It means "the CPU wrote a byte the controller has not yet read"; this
        // model consumes every 0x60/0x64 write synchronously, so the input
        // buffer is always drained by the time the guest re-reads status. The
        // guest's i8042 driver polls IBF==0 before writing the *data* byte of a
        // two-byte command (e.g. Write-CTR 0x60 then the config byte); leaving
        // IBF set while `pending` awaits that data byte would deadlock the write
        // and fail controller init ("Can't write CTR while initializing i8042").
        if self.self_test_passed || (self.command_byte & CMD_BYTE_SYSF) != 0 {
            status |= STATUS_SYSF;
        }
        status
    }

    /// Promote a queued byte into the output buffer if it is currently empty,
    /// updating OBF state and raising the appropriate IRQ.
    ///
    /// Keyboard data has priority over auxiliary data.
    fn refill_output_buffer(&mut self) {
        if self.output_byte.is_some() {
            return;
        }

        if let Some(byte) = self.kbd_queue.pop_front() {
            self.output_byte = Some(byte);
            self.output_is_aux = false;
            if (self.command_byte & CMD_BYTE_KBD_IRQ) != 0 {
                self.kbd_irq_pending = true;
            }
        } else if let Some(byte) = self.aux_queue.pop_front() {
            self.output_byte = Some(byte);
            self.output_is_aux = true;
            if (self.command_byte & CMD_BYTE_AUX_IRQ) != 0 {
                self.aux_irq_pending = true;
            }
        }
    }

    /// Place a controller-generated reply directly into the output buffer.
    ///
    /// Controller replies (self-test result, command-byte read, etc.) jump the
    /// queue. They behave like keyboard data for OBF/IRQ purposes.
    fn reply(&mut self, byte: u8) {
        self.output_byte = Some(byte);
        self.output_is_aux = false;
        if (self.command_byte & CMD_BYTE_KBD_IRQ) != 0 {
            self.kbd_irq_pending = true;
        }
    }

    /// Read the data port (0x60): pop the output buffer, clear OBF, and refill
    /// from the pending queues if more data is waiting.
    fn read_data(&mut self) -> u8 {
        let value = self.output_byte.take().unwrap_or(0);
        self.output_is_aux = false;
        // More data queued? Promote it (which may re-raise an IRQ).
        self.refill_output_buffer();
        value
    }

    /// Handle a write to the data port (0x60).
    fn write_data(&mut self, value: u8) {
        match std::mem::replace(&mut self.pending, PendingWrite::None) {
            PendingWrite::CommandByte => {
                self.command_byte = value;
                // Reflect clock-disable bits into our enable state.
                self.kbd_enabled = (value & CMD_BYTE_KBD_DISABLE) == 0;
                self.aux_enabled = (value & CMD_BYTE_AUX_DISABLE) == 0;
            }
            PendingWrite::OutputPort => {
                self.set_output_port(value);
            }
            PendingWrite::AuxByte => {
                // 0xD4 routed this 0x60 write to the auxiliary device: feed it
                // to the emulated PS/2 mouse, which replies on the aux stream.
                self.mouse_input(value);
            }
            PendingWrite::None => {
                // Plain keyboard command byte from the guest. Most keyboards
                // ACK with 0xFA. We surface that so guest drivers progress.
                if self.kbd_enabled {
                    self.kbd_queue.push_back(0xFA);
                    self.refill_output_buffer();
                }
            }
        }
    }

    /// Apply a new output-port value, tracking A20 and reset-line transitions.
    fn set_output_port(&mut self, value: u8) {
        // Driving the system-reset line (bit 0) low requests a CPU reset.
        if (value & OUTPUT_PORT_RESET_LINE) == 0 {
            self.reset_requested = true;
        }
        self.output_port = value;
    }

    /// Feed a byte to the emulated PS/2 mouse (already routed via 0xD4).
    ///
    /// Implements the standard mouse command set. Every command (and accepted
    /// parameter) is acknowledged with 0xFA on the auxiliary stream; some
    /// commands append further reply bytes (self-test result, device id,
    /// status, etc.).
    fn mouse_input(&mut self, byte: u8) {
        // A parameter byte for a two-byte command takes priority over decoding
        // the byte as a fresh command.
        match std::mem::replace(&mut self.mouse.pending, MousePending::None) {
            MousePending::SampleRate => {
                self.mouse.sample_rate = byte;
                // Slide the magic-knock history and check for 200/100/80, which
                // upgrades the device to an IntelliMouse (scroll wheel).
                self.mouse.rate_history[0] = self.mouse.rate_history[1];
                self.mouse.rate_history[1] = self.mouse.rate_history[2];
                self.mouse.rate_history[2] = byte;
                if self.mouse.rate_history == [200, 100, 80] {
                    self.mouse.device_id = MOUSE_ID_INTELLIMOUSE;
                }
                self.push_mouse_byte(MOUSE_ACK);
                return;
            }
            MousePending::Resolution => {
                // Resolution code 0..=3 selects 1/2/4/8 counts-per-mm.
                self.mouse.resolution = byte & 0x03;
                self.push_mouse_byte(MOUSE_ACK);
                return;
            }
            MousePending::None => {}
        }

        match byte {
            MOUSE_CMD_RESET => {
                // Full reset: defaults, standard id, then BAT + id replies.
                self.mouse.apply_defaults();
                self.mouse.device_id = MOUSE_ID_STANDARD;
                self.mouse.rate_history = [0; 3];
                self.mouse.buttons = 0;
                self.push_mouse_byte(MOUSE_ACK);
                self.push_mouse_byte(MOUSE_SELF_TEST_OK);
                self.push_mouse_byte(self.mouse.device_id);
            }
            MOUSE_CMD_RESEND => {
                // We do not retain the last packet for true resend; ACK so the
                // guest driver makes progress (matches common emulators).
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_SET_DEFAULTS => {
                self.mouse.apply_defaults();
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_DISABLE_REPORTING => {
                self.mouse.reporting = false;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_ENABLE_REPORTING => {
                self.mouse.reporting = true;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_SET_SAMPLE_RATE => {
                self.mouse.pending = MousePending::SampleRate;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_GET_DEVICE_ID => {
                self.push_mouse_byte(MOUSE_ACK);
                self.push_mouse_byte(self.mouse.device_id);
            }
            MOUSE_CMD_SET_REMOTE_MODE => {
                self.mouse.stream_mode = false;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_SET_STREAM_MODE => {
                self.mouse.stream_mode = true;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_READ_DATA => {
                // Remote-mode poll: ACK then a single (idle) movement packet
                // reflecting current button state with zero motion.
                self.push_mouse_byte(MOUSE_ACK);
                let buttons = self.mouse.buttons;
                self.queue_movement_packet(0, 0, buttons, 0);
            }
            MOUSE_CMD_STATUS_REQUEST => {
                self.push_mouse_byte(MOUSE_ACK);
                let (b0, b1, b2) = self.mouse_status_bytes();
                self.push_mouse_byte(b0);
                self.push_mouse_byte(b1);
                self.push_mouse_byte(b2);
            }
            MOUSE_CMD_SET_RESOLUTION => {
                self.mouse.pending = MousePending::Resolution;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_SET_SCALING_2_1 => {
                self.mouse.scaling_2_1 = true;
                self.push_mouse_byte(MOUSE_ACK);
            }
            MOUSE_CMD_SET_SCALING_1_1 => {
                self.mouse.scaling_2_1 = false;
                self.push_mouse_byte(MOUSE_ACK);
            }
            _ => {
                // Unknown mouse command: ACK (most real mice ACK unknown bytes).
                self.push_mouse_byte(MOUSE_ACK);
            }
        }
    }

    /// Build the three bytes returned by a mouse Status Request (0xE9).
    fn mouse_status_bytes(&self) -> (u8, u8, u8) {
        // Byte 0: mode/state flags. bit0 right btn, bit1 middle, bit2 left,
        // bit4 scaling 2:1, bit5 data reporting, bit6 remote (vs stream).
        let mut b0 = 0u8;
        if (self.mouse.buttons & MOUSE_PKT_RIGHT_BTN) != 0 {
            b0 |= 1 << 0;
        }
        if (self.mouse.buttons & MOUSE_PKT_MIDDLE_BTN) != 0 {
            b0 |= 1 << 1;
        }
        if (self.mouse.buttons & MOUSE_PKT_LEFT_BTN) != 0 {
            b0 |= 1 << 2;
        }
        if self.mouse.scaling_2_1 {
            b0 |= 1 << 4;
        }
        if self.mouse.reporting {
            b0 |= 1 << 5;
        }
        if !self.mouse.stream_mode {
            b0 |= 1 << 6;
        }
        // Byte 1: resolution code. Byte 2: sample rate.
        (b0, self.mouse.resolution, self.mouse.sample_rate)
    }

    /// Handle a write to the command port (0x64).
    fn write_command(&mut self, command: u8) {
        match command {
            CMD_READ_CMD_BYTE => {
                let byte = self.command_byte;
                self.reply(byte);
            }
            CMD_WRITE_CMD_BYTE => {
                self.pending = PendingWrite::CommandByte;
            }
            CMD_SELF_TEST => {
                self.self_test_passed = true;
                self.command_byte |= CMD_BYTE_SYSF;
                self.reply(SELF_TEST_OK);
            }
            CMD_TEST_KBD => {
                self.reply(PORT_TEST_OK);
            }
            CMD_TEST_AUX => {
                self.reply(PORT_TEST_OK);
            }
            CMD_DISABLE_KBD => {
                self.kbd_enabled = false;
                self.command_byte |= CMD_BYTE_KBD_DISABLE;
            }
            CMD_ENABLE_KBD => {
                self.kbd_enabled = true;
                self.command_byte &= !CMD_BYTE_KBD_DISABLE;
                self.refill_output_buffer();
            }
            CMD_DISABLE_AUX => {
                self.aux_enabled = false;
                self.command_byte |= CMD_BYTE_AUX_DISABLE;
            }
            CMD_ENABLE_AUX => {
                self.aux_enabled = true;
                self.command_byte &= !CMD_BYTE_AUX_DISABLE;
                self.refill_output_buffer();
            }
            CMD_READ_OUTPUT_PORT => {
                let byte = self.output_port;
                self.reply(byte);
            }
            CMD_WRITE_OUTPUT_PORT => {
                self.pending = PendingWrite::OutputPort;
            }
            CMD_WRITE_AUX => {
                self.pending = PendingWrite::AuxByte;
            }
            cmd if cmd >= CMD_PULSE_BASE => {
                // 0xF0..=0xFF: pulse output lines low. The low nibble selects
                // which lines pulse; an inverted bit 0 means "do not hold reset
                // active". 0xFE => pulse only the reset line (bit 0) low.
                let lines = cmd & 0x0F;
                if (lines & OUTPUT_PORT_RESET_LINE) == 0 {
                    // Bit 0 low in the pulse mask => reset line is pulsed.
                    self.reset_requested = true;
                }
            }
            _ => {
                // Unknown command: ignore (a real controller may NAK).
            }
        }
    }
}

/// Clamp a movement delta to the PS/2 9-bit signed range carried by a packet
/// (sign bit in byte 0, magnitude byte). Returns the clamped value and whether
/// the original delta overflowed that range (setting the packet overflow bit).
fn clamp_delta(delta: i16) -> (i16, bool) {
    if delta > 255 {
        (255, true)
    } else if delta < -256 {
        (-256, true)
    } else {
        (delta, false)
    }
}

impl IoDevice for I8042 {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            DATA_PORT => self.read_data(),
            STATUS_PORT => self.status(),
            _ => 0xFF,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            DATA_PORT => self.write_data(value),
            STATUS_PORT => self.write_command(value),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devices::bus::IoDevice;

    fn fresh() -> I8042 {
        I8042::new()
    }

    // ----- Self-test -------------------------------------------------------

    #[test]
    fn i8042_self_test_returns_0x55() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_SELF_TEST);
        // OBF should be set.
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        // Reading data yields the self-test OK byte.
        assert_eq!(kbc.read(DATA_PORT), SELF_TEST_OK);
        // OBF clears after the read.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        // SYSF reported after a successful self-test.
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_SYSF, 0);
    }

    #[test]
    fn i8042_interface_tests_return_zero() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_TEST_KBD);
        assert_eq!(kbc.read(DATA_PORT), PORT_TEST_OK);
        kbc.write(STATUS_PORT, CMD_TEST_AUX);
        assert_eq!(kbc.read(DATA_PORT), PORT_TEST_OK);
    }

    // ----- Command byte read/write ----------------------------------------

    #[test]
    fn i8042_command_byte_write_then_read() {
        let mut kbc = fresh();
        // Write a fresh command byte: keyboard IRQ on, aux IRQ on, SYSF on.
        let new_byte = CMD_BYTE_KBD_IRQ | CMD_BYTE_AUX_IRQ | CMD_BYTE_SYSF;
        // IBF is never asserted: writes are consumed synchronously, so the
        // guest (which polls IBF==0 before each write) is never blocked.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_IBF, 0);
        kbc.write(STATUS_PORT, CMD_WRITE_CMD_BYTE);
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_IBF, 0);
        kbc.write(DATA_PORT, new_byte);
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_IBF, 0);

        // Read it back via 0x20.
        kbc.write(STATUS_PORT, CMD_READ_CMD_BYTE);
        assert_eq!(kbc.read(DATA_PORT), new_byte);
    }

    #[test]
    fn i8042_ibf_stays_clear_during_two_byte_command() {
        // Regression guard for the boot deadlock: a two-byte command must NOT
        // leave IBF asserted while awaiting its data byte. Linux's i8042 driver
        // polls IBF==0 before writing that data byte, so a stuck IBF makes
        // controller init fail ("Can't write CTR while initializing i8042").
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_WRITE_OUTPUT_PORT);
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_IBF, 0);
        kbc.write(DATA_PORT, 0x02); // output-port value (A20 enabled, reset high)
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_IBF, 0);
    }

    // ----- Scancode FIFO + OBF + IRQ1 gating ------------------------------

    #[test]
    fn i8042_scancode_fifo_sets_obf_and_pops_in_order() {
        let mut kbc = fresh();
        kbc.push_keyboard_scancode(0x1C); // 'A' make code
        kbc.push_keyboard_scancode(0x9C); // 'A' break code

        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert_eq!(kbc.read(DATA_PORT), 0x1C);
        // Next byte promoted from the FIFO; OBF still set.
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert_eq!(kbc.read(DATA_PORT), 0x9C);
        // FIFO drained; OBF clear.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
    }

    #[test]
    fn i8042_irq1_raised_when_enabled() {
        let mut kbc = fresh();
        // Default command byte enables keyboard IRQ.
        assert!(!kbc.keyboard_irq_pending());
        kbc.push_keyboard_scancode(0x1C);
        assert!(kbc.keyboard_irq_pending());
        kbc.clear_keyboard_irq();
        assert!(!kbc.keyboard_irq_pending());
    }

    #[test]
    fn i8042_irq1_gated_off_when_disabled() {
        let mut kbc = fresh();
        // Disable the keyboard IRQ via the command byte.
        kbc.write(STATUS_PORT, CMD_WRITE_CMD_BYTE);
        kbc.write(DATA_PORT, CMD_BYTE_TRANSLATE); // KBD IRQ bit clear
        kbc.push_keyboard_scancode(0x1C);
        // Data still arrives (OBF set) but no IRQ is raised.
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert!(!kbc.keyboard_irq_pending());
    }

    // ----- A20 via output port --------------------------------------------

    #[test]
    fn i8042_a20_gate_via_output_port() {
        let mut kbc = fresh();
        assert!(kbc.a20_enabled()); // Default reset value has A20 on.

        // Disable A20: write output port with bit 1 clear, bit 0 high (no reset).
        kbc.write(STATUS_PORT, CMD_WRITE_OUTPUT_PORT);
        kbc.write(DATA_PORT, OUTPUT_PORT_RESET_LINE); // bit0=1, bit1=0
        assert!(!kbc.a20_enabled());
        assert!(!kbc.reset_requested()); // Reset line was held high.

        // Re-enable A20.
        kbc.write(STATUS_PORT, CMD_WRITE_OUTPUT_PORT);
        kbc.write(DATA_PORT, OUTPUT_PORT_RESET_LINE | OUTPUT_PORT_A20);
        assert!(kbc.a20_enabled());
    }

    #[test]
    fn i8042_read_output_port() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_READ_OUTPUT_PORT);
        assert_eq!(kbc.read(DATA_PORT), OUTPUT_PORT_RESET);
    }

    // ----- Reset request --------------------------------------------------

    #[test]
    fn i8042_pulse_0xfe_requests_reset() {
        let mut kbc = fresh();
        assert!(!kbc.reset_requested());
        kbc.write(STATUS_PORT, 0xFE); // Pulse reset line low.
        assert!(kbc.reset_requested());
        kbc.clear_reset_request();
        assert!(!kbc.reset_requested());
    }

    #[test]
    fn i8042_reset_via_output_port_bit0_low() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_WRITE_OUTPUT_PORT);
        // Drive bit 0 (reset line) low while keeping A20 high.
        kbc.write(DATA_PORT, OUTPUT_PORT_A20);
        assert!(kbc.reset_requested());
    }

    // ----- Aux / mouse routing --------------------------------------------

    #[test]
    fn i8042_aux_byte_sets_obf_and_aux_status() {
        let mut kbc = fresh();
        // Enable the aux IRQ so we can observe IRQ12 gating too.
        kbc.write(STATUS_PORT, CMD_WRITE_CMD_BYTE);
        kbc.write(DATA_PORT, CMD_BYTE_KBD_IRQ | CMD_BYTE_AUX_IRQ);

        kbc.push_mouse_byte(0x29);
        let status = kbc.read(STATUS_PORT);
        assert_ne!(status & STATUS_OBF, 0);
        assert_ne!(status & STATUS_AUX, 0); // Output came from aux device.
        assert!(kbc.aux_irq_pending());
        assert!(!kbc.keyboard_irq_pending());

        assert_eq!(kbc.read(DATA_PORT), 0x29);
        // AUX flag clears once the byte is read.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_AUX, 0);
    }

    #[test]
    fn i8042_write_aux_routes_to_mouse_stream() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_WRITE_CMD_BYTE);
        kbc.write(DATA_PORT, CMD_BYTE_KBD_IRQ | CMD_BYTE_AUX_IRQ);

        // 0xD4 => the next 0x60 write goes to the auxiliary device, which ACKs.
        kbc.write(STATUS_PORT, CMD_WRITE_AUX);
        kbc.write(DATA_PORT, 0xF4); // "enable data reporting" to the mouse
        let status = kbc.read(STATUS_PORT);
        assert_ne!(status & STATUS_AUX, 0);
        assert_eq!(kbc.read(DATA_PORT), 0xFA); // mouse ACK on the aux stream
    }

    #[test]
    fn i8042_aux_irq_gated_off_when_disabled() {
        let mut kbc = fresh();
        // Default command byte has the aux IRQ disabled.
        kbc.push_mouse_byte(0x29);
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert!(!kbc.aux_irq_pending());
    }

    // ----- Enable/disable + keyboard ACK ----------------------------------

    #[test]
    fn i8042_disable_enable_keyboard() {
        let mut kbc = fresh();
        kbc.write(STATUS_PORT, CMD_DISABLE_KBD);
        kbc.push_keyboard_scancode(0x1C);
        // Disabled keyboard accepts no scancodes.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);

        kbc.write(STATUS_PORT, CMD_ENABLE_KBD);
        kbc.push_keyboard_scancode(0x1C);
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert_eq!(kbc.read(DATA_PORT), 0x1C);
    }

    #[test]
    fn i8042_keyboard_command_gets_ack() {
        let mut kbc = fresh();
        // A bare 0x60 data write (no preceding controller command) is a
        // keyboard command; the keyboard ACKs with 0xFA.
        kbc.write(DATA_PORT, 0xFF); // keyboard reset command
        assert_ne!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert_eq!(kbc.read(DATA_PORT), 0xFA);
    }

    #[test]
    fn i8042_unknown_ports_read_ff() {
        let mut kbc = fresh();
        assert_eq!(kbc.read(0x61), 0xFF);
    }

    // ----- PS/2 mouse protocol --------------------------------------------

    /// Enable the auxiliary IRQ in the controller command byte so IRQ12 gating
    /// can be observed.
    fn enable_aux_irq(kbc: &mut I8042) {
        kbc.write(STATUS_PORT, CMD_WRITE_CMD_BYTE);
        kbc.write(DATA_PORT, CMD_BYTE_KBD_IRQ | CMD_BYTE_AUX_IRQ);
    }

    /// Send a single byte to the mouse via 0x64=0xD4 then 0x60=<byte>.
    fn mouse_cmd(kbc: &mut I8042, byte: u8) {
        kbc.write(STATUS_PORT, CMD_WRITE_AUX);
        kbc.write(DATA_PORT, byte);
    }

    /// Read one byte off the auxiliary stream, asserting OBF+AUX are set first.
    fn read_aux(kbc: &mut I8042) -> u8 {
        let status = kbc.read(STATUS_PORT);
        assert_ne!(status & STATUS_OBF, 0, "expected OBF set for aux byte");
        assert_ne!(status & STATUS_AUX, 0, "expected AUX flag for aux byte");
        kbc.read(DATA_PORT)
    }

    #[test]
    fn mouse_reset_sequence_ack_bat_id() {
        let mut kbc = fresh();
        mouse_cmd(&mut kbc, MOUSE_CMD_RESET);
        // Reset returns ACK (0xFA), self-test pass (0xAA), then device id 0x00.
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        assert_eq!(read_aux(&mut kbc), MOUSE_SELF_TEST_OK);
        assert_eq!(read_aux(&mut kbc), MOUSE_ID_STANDARD);
        // Aux stream drained afterwards.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
    }

    #[test]
    fn mouse_get_device_id_default_is_standard() {
        let mut kbc = fresh();
        mouse_cmd(&mut kbc, MOUSE_CMD_GET_DEVICE_ID);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        assert_eq!(read_aux(&mut kbc), MOUSE_ID_STANDARD);
    }

    #[test]
    fn mouse_set_sample_rate_acks_command_and_param() {
        let mut kbc = fresh();
        // Command byte ACK.
        mouse_cmd(&mut kbc, MOUSE_CMD_SET_SAMPLE_RATE);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        // Parameter byte (rate = 100) also ACKs.
        kbc.write(STATUS_PORT, CMD_WRITE_AUX);
        kbc.write(DATA_PORT, 100);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
    }

    #[test]
    fn mouse_enable_reporting_then_movement_produces_packet_and_irq12() {
        let mut kbc = fresh();
        enable_aux_irq(&mut kbc);

        // Enable data reporting; consume the ACK.
        mouse_cmd(&mut kbc, MOUSE_CMD_ENABLE_REPORTING);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        kbc.clear_aux_irq();

        // Move right (+5, dx) and down (-3, dy), left button held.
        kbc.push_mouse_movement(5, -3, MOUSE_PKT_LEFT_BTN, 0);

        // The packet promotion raises IRQ12.
        assert!(kbc.aux_irq_pending(), "movement should raise IRQ12");

        // Byte 0: always-one bit, left button, Y sign (dy negative).
        let b0 = read_aux(&mut kbc);
        assert_ne!(b0 & MOUSE_PKT_ALWAYS_ONE, 0);
        assert_ne!(b0 & MOUSE_PKT_LEFT_BTN, 0);
        assert_eq!(b0 & MOUSE_PKT_RIGHT_BTN, 0);
        assert_eq!(b0 & MOUSE_PKT_X_SIGN, 0); // dx positive
        assert_ne!(b0 & MOUSE_PKT_Y_SIGN, 0); // dy negative

        // Byte 1: dx = 5.
        assert_eq!(read_aux(&mut kbc), 5);
        // Byte 2: dy = -3 as a two's-complement byte.
        assert_eq!(read_aux(&mut kbc), (-3i16 & 0xFF) as u8);
        // Standard mouse: exactly 3 bytes, stream now drained.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
    }

    #[test]
    fn mouse_no_packet_when_reporting_disabled() {
        let mut kbc = fresh();
        enable_aux_irq(&mut kbc);
        // Reporting is off by default; movement should produce nothing.
        kbc.push_mouse_movement(10, 10, 0, 0);
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        assert!(!kbc.aux_irq_pending());
    }

    #[test]
    fn mouse_intellimouse_magic_sequence_switches_id_and_packet_size() {
        let mut kbc = fresh();

        // The 200/100/80 set-sample-rate knock unlocks IntelliMouse.
        for rate in [200u8, 100, 80] {
            mouse_cmd(&mut kbc, MOUSE_CMD_SET_SAMPLE_RATE);
            assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
            kbc.write(STATUS_PORT, CMD_WRITE_AUX);
            kbc.write(DATA_PORT, rate);
            assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        }

        // Get Device ID now reports 0x03.
        mouse_cmd(&mut kbc, MOUSE_CMD_GET_DEVICE_ID);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        assert_eq!(read_aux(&mut kbc), MOUSE_ID_INTELLIMOUSE);

        // Enable reporting, then a movement now yields a 4-byte packet.
        mouse_cmd(&mut kbc, MOUSE_CMD_ENABLE_REPORTING);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);

        kbc.push_mouse_movement(1, 1, 0, -2);
        let _b0 = read_aux(&mut kbc);
        assert_eq!(read_aux(&mut kbc), 1); // dx
        assert_eq!(read_aux(&mut kbc), 1); // dy
        // Byte 3: scroll delta -2 in the low nibble.
        assert_eq!(read_aux(&mut kbc), (-2i8 as u8) & 0x0F);
        // Exactly 4 bytes; stream drained.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
    }

    #[test]
    fn mouse_status_request_returns_three_bytes() {
        let mut kbc = fresh();
        // Enable reporting and 2:1 scaling so status bits are observable.
        mouse_cmd(&mut kbc, MOUSE_CMD_ENABLE_REPORTING);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        mouse_cmd(&mut kbc, MOUSE_CMD_SET_SCALING_2_1);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);

        mouse_cmd(&mut kbc, MOUSE_CMD_STATUS_REQUEST);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        let b0 = read_aux(&mut kbc);
        let _resolution = read_aux(&mut kbc);
        let sample_rate = read_aux(&mut kbc);
        // Stream drained after exactly three status bytes.
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);

        // bit5 = data reporting on, bit4 = 2:1 scaling on, bit6 = remote off.
        assert_ne!(b0 & (1 << 5), 0, "reporting bit");
        assert_ne!(b0 & (1 << 4), 0, "scaling 2:1 bit");
        assert_eq!(b0 & (1 << 6), 0, "stream mode (remote bit clear)");
        // Default sample rate is 100.
        assert_eq!(sample_rate, 100);
    }

    #[test]
    fn mouse_set_resolution_acks_command_and_param() {
        let mut kbc = fresh();
        mouse_cmd(&mut kbc, MOUSE_CMD_SET_RESOLUTION);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        kbc.write(STATUS_PORT, CMD_WRITE_AUX);
        kbc.write(DATA_PORT, 0x03); // 8 counts/mm
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
    }

    #[test]
    fn mouse_simple_commands_each_ack() {
        let mut kbc = fresh();
        for cmd in [
            MOUSE_CMD_SET_DEFAULTS,
            MOUSE_CMD_DISABLE_REPORTING,
            MOUSE_CMD_SET_REMOTE_MODE,
            MOUSE_CMD_SET_STREAM_MODE,
            MOUSE_CMD_SET_SCALING_1_1,
            MOUSE_CMD_SET_SCALING_2_1,
            MOUSE_CMD_RESEND,
        ] {
            mouse_cmd(&mut kbc, cmd);
            assert_eq!(read_aux(&mut kbc), MOUSE_ACK, "command {cmd:#x} should ACK");
            // Each ACK drains; no trailing bytes.
            assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
        }
    }

    #[test]
    fn mouse_remote_mode_read_data_returns_packet() {
        let mut kbc = fresh();
        // Switch to remote mode; reporting state does not gate Read Data.
        mouse_cmd(&mut kbc, MOUSE_CMD_SET_REMOTE_MODE);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);

        mouse_cmd(&mut kbc, MOUSE_CMD_READ_DATA);
        assert_eq!(read_aux(&mut kbc), MOUSE_ACK);
        // Idle packet: always-one bit set, zero motion.
        let b0 = read_aux(&mut kbc);
        assert_ne!(b0 & MOUSE_PKT_ALWAYS_ONE, 0);
        assert_eq!(read_aux(&mut kbc), 0); // dx
        assert_eq!(read_aux(&mut kbc), 0); // dy
        assert_eq!(kbc.read(STATUS_PORT) & STATUS_OBF, 0);
    }
}
