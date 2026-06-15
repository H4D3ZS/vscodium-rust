//! PCI host bridge with type-1 configuration space access.
//!
//! Implements the classic x86 PCI configuration mechanism #1:
//!   * `0xCF8` CONFIG_ADDRESS: a 32-bit latch selecting (bus, device, function,
//!     register) of the config dword to access.
//!   * `0xCFC..=0xCFF` CONFIG_DATA: byte/word/dword reads & writes that route to
//!     the addressed config dword, honoring the byte offset within the dword.
//!
//! A registry of [`ConfigSpace`] entries keyed by (bus, device, function) lets
//! other PCI devices be attached. The host bridge itself lives at 00:00.0.
//! Unoccupied slots read back as all-ones (0xFFFFFFFF), exactly as real
//! hardware reports "no device present".
//!
//! BAR sizing is supported: when the guest writes 0xFFFFFFFF to a BAR register
//! and reads it back, the value is masked according to the BAR's registered
//! size and type bits, which is how firmware discovers the size of each region.

use super::bus::{IoDevice, MmioDevice};
use std::collections::HashMap;

/// PCI configuration space ports.
pub const PCI_CONFIG_ADDRESS: u16 = 0xcf8;
pub const PCI_CONFIG_DATA: u16 = 0xcfc;

/// Number of bytes in a single function's PCI configuration space.
pub const CONFIG_SPACE_SIZE: usize = 256;

/// Bit 31 of CONFIG_ADDRESS: when set, CONFIG_DATA accesses are enabled.
const CONFIG_ADDRESS_ENABLE: u32 = 1 << 31;

/// Offset of the first Base Address Register (BAR0) within config space.
const BAR0_OFFSET: usize = 0x10;
/// Header-type-0 has 6 BARs (BAR0..BAR5).
const NUM_BARS: usize = 6;

/// Describes a single Base Address Register so that BAR sizing works.
#[derive(Clone, Copy, Debug)]
pub struct Bar {
    /// Region length in bytes. Must be a power of two (PCI requirement). A
    /// length of 0 means the BAR is unused (decodes to 0).
    pub size: u32,
    /// Whether this is an I/O-space BAR (true) or a memory-space BAR (false).
    pub io: bool,
    /// For memory BARs: whether the region is prefetchable.
    pub prefetchable: bool,
    /// For memory BARs: whether it is a 64-bit BAR (consuming the next BAR slot
    /// as the high dword). I/O BARs are always 32-bit.
    pub bits64: bool,
}

impl Bar {
    /// A 32-bit memory BAR of the given size.
    pub fn mem32(size: u32) -> Self {
        Bar {
            size,
            io: false,
            prefetchable: false,
            bits64: false,
        }
    }

    /// A 32-bit prefetchable memory BAR of the given size.
    pub fn mem32_prefetch(size: u32) -> Self {
        Bar {
            size,
            io: false,
            prefetchable: true,
            bits64: false,
        }
    }

    /// A 64-bit memory BAR of the given size (consumes the next BAR slot).
    pub fn mem64(size: u32) -> Self {
        Bar {
            size,
            io: false,
            prefetchable: false,
            bits64: true,
        }
    }

    /// An I/O-space BAR of the given size.
    pub fn io(size: u32) -> Self {
        Bar {
            size,
            io: true,
            prefetchable: false,
            bits64: false,
        }
    }

    /// The low "type" bits stored in the BAR's least-significant bits.
    ///   * I/O BAR: bit0 = 1.
    ///   * Memory BAR: bit0 = 0, bits[2:1] = type (0 = 32-bit, 2 = 64-bit),
    ///     bit3 = prefetchable.
    fn type_bits(&self) -> u32 {
        if self.io {
            // Bit 0 = 1 marks I/O space; bit 1 is reserved (read 0).
            0x1
        } else {
            let mut bits = 0u32;
            if self.bits64 {
                bits |= 0b100; // type = 10b in bits[2:1]
            }
            if self.prefetchable {
                bits |= 0b1000;
            }
            bits
        }
    }

    /// The address mask reported when the guest probes the BAR with all-ones.
    /// The address bits above the size return `!(size-1)`, the low type bits
    /// are preserved, and the bits in between (within the size) read as zero.
    fn size_mask(&self) -> u32 {
        if self.size == 0 {
            return 0;
        }
        let addr_mask = !(self.size.wrapping_sub(1));
        // For I/O BARs the upper bits beyond bit 1 are address bits; for memory
        // BARs the address bits start at bit 4. The `size` is a power of two
        // that is always >= the type-bit region, so masking is uniform.
        (addr_mask & !low_bits_mask(self.io)) | self.type_bits()
    }

    /// Apply the BAR's read-back semantics to a raw 32-bit value the guest has
    /// written into the BAR dword. A raw value with all address bits set is
    /// treated as a size probe and returns [`size_mask`]; otherwise the value
    /// is the programmed base with its type bits forced to the correct value.
    fn read_back(&self, raw: u32) -> u32 {
        let low = low_bits_mask(self.io);
        // A "probe" sets every address bit (the low type bits are don't-care
        // during a probe). Detect it by checking the address-bit region.
        if (raw & !low) == (0xffff_ffffu32 & !low) {
            self.size_mask()
        } else {
            (raw & !low) | self.type_bits()
        }
    }
}

/// Mask covering the low type bits that are never part of the address: bits
/// [1:0] for I/O BARs, bits [3:0] for memory BARs.
fn low_bits_mask(io: bool) -> u32 {
    if io { 0b11 } else { 0b1111 }
}

/// A single PCI function's 256-byte configuration space plus BAR metadata used
/// for sizing. Registers a function via [`PciStub::add_function`].
#[derive(Clone)]
pub struct ConfigSpace {
    bytes: [u8; CONFIG_SPACE_SIZE],
    bars: [Option<Bar>; NUM_BARS],
}

impl ConfigSpace {
    /// Create an empty config space (all zero). The header type defaults to 0.
    pub fn new() -> Self {
        ConfigSpace {
            bytes: [0u8; CONFIG_SPACE_SIZE],
            bars: [None; NUM_BARS],
        }
    }

    /// Build a typical header-type-0 device config space.
    pub fn device(vendor_id: u16, device_id: u16, class_code: u32, header_type: u8) -> Self {
        let mut cs = ConfigSpace::new();
        cs.set_u16(0x00, vendor_id);
        cs.set_u16(0x02, device_id);
        // class_code occupies bytes 0x09 (base), 0x0a (sub), 0x0b? No:
        // 0x08 = revision, 0x09..0x0b = prog-if/subclass/class.
        cs.set_u8(0x09, (class_code & 0xff) as u8); // prog IF
        cs.set_u8(0x0a, ((class_code >> 8) & 0xff) as u8); // subclass
        cs.set_u8(0x0b, ((class_code >> 16) & 0xff) as u8); // base class
        cs.set_u8(0x0e, header_type);
        cs
    }

    /// Set a byte in config space.
    pub fn set_u8(&mut self, offset: usize, value: u8) {
        self.bytes[offset] = value;
    }

    /// Set a little-endian u16 in config space.
    pub fn set_u16(&mut self, offset: usize, value: u16) {
        self.bytes[offset] = (value & 0xff) as u8;
        self.bytes[offset + 1] = (value >> 8) as u8;
    }

    /// Set a little-endian u32 in config space.
    pub fn set_u32(&mut self, offset: usize, value: u32) {
        self.bytes[offset] = (value & 0xff) as u8;
        self.bytes[offset + 1] = ((value >> 8) & 0xff) as u8;
        self.bytes[offset + 2] = ((value >> 16) & 0xff) as u8;
        self.bytes[offset + 3] = ((value >> 24) & 0xff) as u8;
    }

    /// Read a little-endian u32 from config space.
    fn get_u32(&self, offset: usize) -> u32 {
        (self.bytes[offset] as u32)
            | ((self.bytes[offset + 1] as u32) << 8)
            | ((self.bytes[offset + 2] as u32) << 16)
            | ((self.bytes[offset + 3] as u32) << 24)
    }

    /// Register a BAR descriptor for sizing. `index` is 0..=5 (BAR0..BAR5).
    /// The BAR's low type bits are reflected into config space immediately so
    /// that reads before any probe still report the correct type.
    pub fn set_bar(&mut self, index: usize, bar: Bar) {
        assert!(index < NUM_BARS, "BAR index out of range");
        self.bars[index] = Some(bar);
        // Reflect the type bits into the config dword so reads see them.
        let off = BAR0_OFFSET + index * 4;
        let current = self.get_u32(off) & !low_bits_mask(bar.io);
        self.set_u32(off, current | bar.type_bits());
    }

    /// Builder-style BAR setter.
    pub fn with_bar(mut self, index: usize, bar: Bar) -> Self {
        self.set_bar(index, bar);
        self
    }

    /// Index of the BAR register that contains `offset`, if `offset` lies
    /// within the 6-BAR window and a BAR is registered there.
    fn bar_at(&self, offset: usize) -> Option<usize> {
        if offset < BAR0_OFFSET {
            return None;
        }
        let idx = (offset - BAR0_OFFSET) / 4;
        if idx < NUM_BARS && self.bars[idx].is_some() {
            Some(idx)
        } else {
            None
        }
    }
}

impl Default for ConfigSpace {
    fn default() -> Self {
        ConfigSpace::new()
    }
}

/// Key identifying a function on the PCI topology.
type FunctionKey = (u8, u8, u8);

/// PCI host bridge providing mechanism #1 configuration access.
///
/// The public name `PciStub` is retained for backwards-compatible registration,
/// but this is now a real host bridge rather than a stub.
pub struct PciStub {
    /// Latched CONFIG_ADDRESS value (port 0xCF8).
    address: u32,
    /// Registered functions keyed by (bus, device, function).
    functions: HashMap<FunctionKey, ConfigSpace>,
    /// Endpoints whose BAR-mapped register block routes to a device handler.
    endpoints: Vec<PciEndpoint>,
}

/// A BAR-mapped device handler living behind the host bridge.
enum PciHandler {
    Mmio(Box<dyn MmioDevice>),
    Pio(Box<dyn IoDevice>),
}

/// A PCI endpoint whose register block is reachable through one of its BARs.
/// The handler is fed BAR-*relative* offsets, so device models are constructed
/// with a zero base and the dynamic BAR address is applied here — letting the
/// guest reprogram the BAR freely.
struct PciEndpoint {
    key: FunctionKey,
    handler: PciHandler,
    /// BAR index (0..=5) whose window maps the handler.
    bar_index: usize,
    /// BAR window size in bytes (power of two).
    bar_size: u64,
    /// Whether the mapping BAR is I/O space (else memory space).
    io: bool,
    /// Currently decoded + enabled base address of the BAR, or 0 when the BAR is
    /// unprogrammed, mid-size-probe, or its decode is disabled in COMMAND.
    current_base: u64,
}

impl PciStub {
    /// Construct a host bridge with the 440FX bridge at 00:00.0.
    pub fn new() -> Self {
        let mut pci = PciStub {
            address: 0,
            functions: HashMap::new(),
            endpoints: Vec::new(),
        };
        // Host bridge (Intel 440FX PMC) at bus 0, device 0, function 0.
        // Vendor 0x8086 (Intel), device 0x1237 (440FX), class 0x060000
        // (bridge / host bridge), header type 0.
        let bridge = ConfigSpace::device(0x8086, 0x1237, 0x06_00_00, 0x00);
        pci.add_function(0, 0, 0, bridge);
        pci
    }

    /// Attach a PCI function's configuration space at (bus, device, function).
    pub fn add_function(&mut self, bus: u8, device: u8, function: u8, cs: ConfigSpace) {
        self.functions.insert((bus, device, function), cs);
    }

    /// Attach an endpoint with a memory-BAR-mapped MMIO register block. The
    /// handler receives BAR-relative offsets; build the device model with a zero
    /// base. `bar_index` must match a memory BAR registered on `cs`.
    pub fn attach_mmio(
        &mut self,
        bus: u8,
        device: u8,
        function: u8,
        cs: ConfigSpace,
        bar_index: usize,
        handler: Box<dyn MmioDevice>,
    ) {
        let bar_size = cs.bars[bar_index].map(|b| b.size as u64).unwrap_or(0);
        let key = (bus, device, function);
        self.functions.insert(key, cs);
        self.endpoints.push(PciEndpoint {
            key,
            handler: PciHandler::Mmio(handler),
            bar_index,
            bar_size,
            io: false,
            current_base: 0,
        });
        self.recompute_bases();
    }

    /// Attach an endpoint with an I/O-BAR-mapped register block. The handler
    /// receives BAR-relative ports; build the device model with a zero base.
    pub fn attach_pio(
        &mut self,
        bus: u8,
        device: u8,
        function: u8,
        cs: ConfigSpace,
        bar_index: usize,
        handler: Box<dyn IoDevice>,
    ) {
        let bar_size = cs.bars[bar_index].map(|b| b.size as u64).unwrap_or(0);
        let key = (bus, device, function);
        self.functions.insert(key, cs);
        self.endpoints.push(PciEndpoint {
            key,
            handler: PciHandler::Pio(handler),
            bar_index,
            bar_size,
            io: true,
            current_base: 0,
        });
        self.recompute_bases();
    }

    /// Recompute every endpoint's live BAR base from its config space. Called
    /// after any config write (cheap — there are only a handful of endpoints).
    fn recompute_bases(&mut self) {
        for idx in 0..self.endpoints.len() {
            let ep = &self.endpoints[idx];
            let (cmd, raw) = match self.functions.get(&ep.key) {
                Some(cs) => (
                    (cs.get_u32(0x04) & 0xffff) as u16,
                    cs.get_u32(BAR0_OFFSET + ep.bar_index * 4),
                ),
                None => (0, 0),
            };
            // COMMAND bit 0 = I/O decode enable, bit 1 = memory decode enable.
            let decode = if ep.io {
                cmd & 0x1 != 0
            } else {
                cmd & 0x2 != 0
            };
            let low = low_bits_mask(ep.io);
            let size = ep.bar_size.max(1) as u32;
            let addr_mask = !(size.wrapping_sub(1));
            let base = (raw & addr_mask & !low) as u64;
            // Treat an all-ones size probe (or zero/disabled) as "not routable".
            let probing = (raw & !low) == (0xffff_ffffu32 & !low);
            self.endpoints[idx].current_base = if decode && base != 0 && !probing {
                base
            } else {
                0
            };
        }
    }

    /// The smallest..largest span covered by any currently-active *memory* BAR,
    /// or `None` when no memory endpoint is decoding. Used by the emulator MMU
    /// to know which physical range to divert from RAM to PCI.
    pub fn mmio_aperture(&self) -> Option<(u64, u64)> {
        let mut lo = u64::MAX;
        let mut hi = 0u64;
        for ep in &self.endpoints {
            if ep.io || ep.current_base == 0 {
                continue;
            }
            lo = lo.min(ep.current_base);
            hi = hi.max(ep.current_base + ep.bar_size);
        }
        if hi > lo { Some((lo, hi)) } else { None }
    }

    /// Route an MMIO read to the endpoint whose memory BAR covers `addr`.
    /// Returns true (and fills `data`) when handled.
    pub fn mmio_read(&mut self, addr: u64, data: &mut [u8]) -> bool {
        for ep in self.endpoints.iter_mut() {
            if ep.io || ep.current_base == 0 {
                continue;
            }
            let end = ep.current_base + ep.bar_size;
            if addr >= ep.current_base && addr < end {
                if let PciHandler::Mmio(h) = &mut ep.handler {
                    h.read(addr - ep.current_base, data);
                    return true;
                }
            }
        }
        false
    }

    /// Route an MMIO write to the endpoint whose memory BAR covers `addr`.
    pub fn mmio_write(&mut self, addr: u64, data: &[u8]) -> bool {
        for ep in self.endpoints.iter_mut() {
            if ep.io || ep.current_base == 0 {
                continue;
            }
            let end = ep.current_base + ep.bar_size;
            if addr >= ep.current_base && addr < end {
                if let PciHandler::Mmio(h) = &mut ep.handler {
                    h.write(addr - ep.current_base, data);
                    return true;
                }
            }
        }
        false
    }

    /// Route an I/O-port read to the endpoint whose I/O BAR covers `port`.
    pub fn io_read(&mut self, port: u16) -> Option<u8> {
        for ep in self.endpoints.iter_mut() {
            if !ep.io || ep.current_base == 0 {
                continue;
            }
            let end = ep.current_base + ep.bar_size;
            if (port as u64) >= ep.current_base && (port as u64) < end {
                if let PciHandler::Pio(h) = &mut ep.handler {
                    return Some(h.read(port - ep.current_base as u16));
                }
            }
        }
        None
    }

    /// Route an I/O-port write to the endpoint whose I/O BAR covers `port`.
    /// Returns true when handled.
    pub fn io_write(&mut self, port: u16, value: u8) -> bool {
        for ep in self.endpoints.iter_mut() {
            if !ep.io || ep.current_base == 0 {
                continue;
            }
            let end = ep.current_base + ep.bar_size;
            if (port as u64) >= ep.current_base && (port as u64) < end {
                if let PciHandler::Pio(h) = &mut ep.handler {
                    h.write(port - ep.current_base as u16, value);
                    return true;
                }
            }
        }
        false
    }

    /// Decode the currently latched CONFIG_ADDRESS into its components.
    /// Returns `None` when the enable bit (bit 31) is clear.
    fn decode_address(&self) -> Option<DecodedAddress> {
        if self.address & CONFIG_ADDRESS_ENABLE == 0 {
            return None;
        }
        Some(DecodedAddress {
            bus: ((self.address >> 16) & 0xff) as u8,
            device: ((self.address >> 11) & 0x1f) as u8,
            function: ((self.address >> 8) & 0x07) as u8,
            // register index in dwords -> byte offset (bits [7:2]).
            register: (self.address & 0xfc) as usize,
        })
    }

    /// Read the addressed config dword for the currently latched address,
    /// applying BAR-sizing semantics. Returns 0xFFFFFFFF for unoccupied slots
    /// or when the address is not enabled.
    fn read_config_dword(&self) -> u32 {
        let decoded = match self.decode_address() {
            Some(d) => d,
            None => return 0xffff_ffff,
        };
        let cs = match self
            .functions
            .get(&(decoded.bus, decoded.device, decoded.function))
        {
            Some(cs) => cs,
            None => return 0xffff_ffff,
        };
        let raw = cs.get_u32(decoded.register);
        // If the register is a registered BAR, apply BAR read-back semantics
        // (size-mask on probe, type bits forced) to the raw stored value.
        match cs.bar_at(decoded.register) {
            Some(idx) => cs.bars[idx].expect("bar_at guarantees Some").read_back(raw),
            None => raw,
        }
    }

    /// Write `value` into the addressed config dword. For BAR registers the
    /// raw value is stored verbatim (including an all-ones size probe); the
    /// read path applies the size-mask / type-bit semantics. This keeps the
    /// logic correct even when wide accesses arrive as a sequence of byte
    /// writes via the IoBus.
    fn write_config_dword(&mut self, value: u32) {
        let decoded = match self.decode_address() {
            Some(d) => d,
            None => return,
        };
        let key = (decoded.bus, decoded.device, decoded.function);
        let cs = match self.functions.get_mut(&key) {
            Some(cs) => cs,
            None => return,
        };
        cs.set_u32(decoded.register, value);
        // A BAR or COMMAND write may change which physical/IO addresses an
        // endpoint decodes; keep the live bases in sync.
        if !self.endpoints.is_empty() {
            self.recompute_bases();
        }
    }
}

impl Default for PciStub {
    fn default() -> Self {
        PciStub::new()
    }
}

/// Decoded form of the CONFIG_ADDRESS latch.
struct DecodedAddress {
    bus: u8,
    device: u8,
    function: u8,
    /// Dword-aligned byte offset into config space (bits [7:2] of the address).
    register: usize,
}

impl IoDevice for PciStub {
    fn read(&mut self, port: u16) -> u8 {
        match port {
            // CONFIG_ADDRESS: return the latched value, byte by byte.
            PCI_CONFIG_ADDRESS..=0xcfb => {
                let offset = (port - PCI_CONFIG_ADDRESS) as usize;
                ((self.address >> (offset * 8)) & 0xff) as u8
            }
            // CONFIG_DATA: route to the addressed config dword, honoring the
            // byte offset within the dword (port 0xCFC..=0xCFF maps to byte
            // 0..=3 of the dword).
            PCI_CONFIG_DATA..=0xcff => {
                let byte = (port - PCI_CONFIG_DATA) as usize;
                let dword = self.read_config_dword();
                ((dword >> (byte * 8)) & 0xff) as u8
            }
            _ => 0xff,
        }
    }

    fn write(&mut self, port: u16, value: u8) {
        match port {
            // CONFIG_ADDRESS latch: assemble the 32-bit value byte by byte.
            PCI_CONFIG_ADDRESS..=0xcfb => {
                let offset = (port - PCI_CONFIG_ADDRESS) as usize;
                let mask = !(0xffu32 << (offset * 8));
                self.address = (self.address & mask) | ((value as u32) << (offset * 8));
            }
            // CONFIG_DATA: merge the byte into the addressed dword at the
            // correct byte offset, then write it back (handles 1/2/4-byte
            // accesses since the IoBus decomposes wider accesses into bytes).
            PCI_CONFIG_DATA..=0xcff => {
                let byte = (port - PCI_CONFIG_DATA) as usize;
                let mut dword = self.read_raw_config_dword();
                let shift = byte * 8;
                dword = (dword & !(0xffu32 << shift)) | ((value as u32) << shift);
                self.write_config_dword(dword);
            }
            _ => {}
        }
    }
}

impl PciStub {
    /// Like [`read_config_dword`] but without BAR-sizing fallback to all-ones
    /// for unoccupied slots; used as the merge base for partial writes so that
    /// a byte write does not clobber the rest of the dword. For unoccupied
    /// slots this returns 0 (a subsequent write is a no-op anyway).
    fn read_raw_config_dword(&self) -> u32 {
        let decoded = match self.decode_address() {
            Some(d) => d,
            None => return 0,
        };
        match self
            .functions
            .get(&(decoded.bus, decoded.device, decoded.function))
        {
            Some(cs) => cs.get_u32(decoded.register),
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a CONFIG_ADDRESS latch value from its components.
    fn make_address(bus: u8, device: u8, function: u8, register: u8) -> u32 {
        CONFIG_ADDRESS_ENABLE
            | ((bus as u32) << 16)
            | (((device as u32) & 0x1f) << 11)
            | (((function as u32) & 0x07) << 8)
            | ((register as u32) & 0xfc)
    }

    /// Write a 32-bit value to CONFIG_ADDRESS via the byte-wise IoDevice API.
    fn set_address(pci: &mut PciStub, addr: u32) {
        for i in 0..4 {
            pci.write(PCI_CONFIG_ADDRESS + i, (addr >> (i * 8)) as u8);
        }
    }

    /// Read a dword from CONFIG_DATA via the byte-wise IoDevice API.
    fn read_data_dword(pci: &mut PciStub) -> u32 {
        let mut v = 0u32;
        for i in 0..4 {
            v |= (pci.read(PCI_CONFIG_DATA + i) as u32) << (i * 8);
        }
        v
    }

    /// Write a dword to CONFIG_DATA via the byte-wise IoDevice API.
    fn write_data_dword(pci: &mut PciStub, value: u32) {
        for i in 0..4 {
            pci.write(PCI_CONFIG_DATA + i, (value >> (i * 8)) as u8);
        }
    }

    #[test]
    fn address_latch_decode() {
        let mut pci = PciStub::new();
        let addr = make_address(0x12, 0x1f, 0x07, 0x3c);
        set_address(&mut pci, addr);
        assert_eq!(pci.address, addr);

        let decoded = pci.decode_address().expect("enable bit is set");
        assert_eq!(decoded.bus, 0x12);
        assert_eq!(decoded.device, 0x1f);
        assert_eq!(decoded.function, 0x07);
        assert_eq!(decoded.register, 0x3c);

        // With the enable bit clear, decode returns None.
        set_address(&mut pci, addr & !CONFIG_ADDRESS_ENABLE);
        assert!(pci.decode_address().is_none());
    }

    #[test]
    fn address_latch_byte_writes_assemble() {
        let mut pci = PciStub::new();
        // Write the latch one byte at a time and confirm assembly.
        pci.write(0xcf8, 0x08);
        pci.write(0xcf9, 0x00);
        pci.write(0xcfa, 0x00);
        pci.write(0xcfb, 0x80);
        assert_eq!(pci.address, 0x8000_0008);
        // And byte reads return the same bytes.
        assert_eq!(pci.read(0xcf8), 0x08);
        assert_eq!(pci.read(0xcfb), 0x80);
    }

    #[test]
    fn host_bridge_vendor_device_class() {
        let mut pci = PciStub::new();

        // Register 0x00: device_id:vendor_id.
        set_address(&mut pci, make_address(0, 0, 0, 0x00));
        let dword = read_data_dword(&mut pci);
        assert_eq!(dword & 0xffff, 0x8086, "vendor id");
        assert_eq!((dword >> 16) & 0xffff, 0x1237, "device id");

        // Register 0x08: class_code:revision. Class lives in bytes [1..3].
        set_address(&mut pci, make_address(0, 0, 0, 0x08));
        let dword = read_data_dword(&mut pci);
        let class = (dword >> 8) & 0xff_ffff;
        assert_eq!(class, 0x06_00_00, "host bridge class code");

        // Header type at 0x0e == 0.
        set_address(&mut pci, make_address(0, 0, 0, 0x0c));
        let dword = read_data_dword(&mut pci);
        let header_type = (dword >> 16) & 0xff;
        assert_eq!(header_type, 0x00);
    }

    #[test]
    fn config_data_byte_word_dword_access() {
        let mut pci = PciStub::new();
        set_address(&mut pci, make_address(0, 0, 0, 0x00));

        // Byte access at each offset within the dword (0xCFC..=0xCFF).
        assert_eq!(pci.read(0xcfc), 0x86); // vendor low
        assert_eq!(pci.read(0xcfd), 0x80); // vendor high
        assert_eq!(pci.read(0xcfe), 0x37); // device low
        assert_eq!(pci.read(0xcff), 0x12); // device high

        // Word access: vendor at 0xCFC, device at 0xCFE.
        let vendor = (pci.read(0xcfc) as u16) | ((pci.read(0xcfd) as u16) << 8);
        assert_eq!(vendor, 0x8086);
        let device = (pci.read(0xcfe) as u16) | ((pci.read(0xcff) as u16) << 8);
        assert_eq!(device, 0x1237);

        // Dword access.
        assert_eq!(read_data_dword(&mut pci), 0x1237_8086);
    }

    #[test]
    fn config_data_partial_write_preserves_dword() {
        let mut pci = PciStub::new();
        // Use a scratch register that is freely writable (command/status at
        // 0x04 is fine for the bridge in our model since it has no special
        // handling). Point at register 0x40 (device-specific, all zero).
        set_address(&mut pci, make_address(0, 0, 0, 0x40));
        write_data_dword(&mut pci, 0xdead_beef);
        assert_eq!(read_data_dword(&mut pci), 0xdead_beef);

        // Now overwrite only the second byte (offset 0xCFD) and confirm the
        // rest of the dword is preserved.
        pci.write(0xcfd, 0x11);
        assert_eq!(read_data_dword(&mut pci), 0xdead_11ef);

        // Word write at the high half.
        pci.write(0xcfe, 0x22);
        pci.write(0xcff, 0x33);
        assert_eq!(read_data_dword(&mut pci), 0x3322_11ef);
    }

    #[test]
    fn unoccupied_slot_reads_all_ones() {
        let mut pci = PciStub::new();
        // Device 5 is not registered.
        set_address(&mut pci, make_address(0, 5, 0, 0x00));
        assert_eq!(read_data_dword(&mut pci), 0xffff_ffff);

        // A different unoccupied bus too.
        set_address(&mut pci, make_address(3, 0, 0, 0x00));
        assert_eq!(read_data_dword(&mut pci), 0xffff_ffff);

        // Reads with the enable bit clear also report all-ones.
        set_address(
            &mut pci,
            make_address(0, 0, 0, 0x00) & !CONFIG_ADDRESS_ENABLE,
        );
        assert_eq!(read_data_dword(&mut pci), 0xffff_ffff);
    }

    #[test]
    fn add_function_registers_device() {
        let mut pci = PciStub::new();
        let dev = ConfigSpace::device(0x1af4, 0x1000, 0x02_00_00, 0x00);
        pci.add_function(0, 3, 0, dev);

        set_address(&mut pci, make_address(0, 3, 0, 0x00));
        let dword = read_data_dword(&mut pci);
        assert_eq!(dword & 0xffff, 0x1af4); // virtio vendor
        assert_eq!((dword >> 16) & 0xffff, 0x1000); // virtio-net device
    }

    #[test]
    fn bar_sizing_mem32() {
        let mut pci = PciStub::new();
        // 64 KiB 32-bit memory BAR at BAR0.
        let dev =
            ConfigSpace::device(0x1af4, 0x1000, 0x02_00_00, 0x00).with_bar(0, Bar::mem32(0x1_0000));
        pci.add_function(0, 4, 0, dev);

        // Before probing, BAR0 reads back with just the type bits (memory,
        // 32-bit, non-prefetchable == 0).
        set_address(&mut pci, make_address(0, 4, 0, BAR0_OFFSET as u8));
        assert_eq!(read_data_dword(&mut pci), 0x0000_0000);

        // Probe: write all-ones, read the size mask.
        write_data_dword(&mut pci, 0xffff_ffff);
        let mask = read_data_dword(&mut pci);
        // 64 KiB => size bits low 16 are zero, address bits above are ones,
        // and the low 4 type bits are zero (mem, 32-bit, non-prefetch).
        assert_eq!(mask, 0xffff_0000);

        // Programming a real base address keeps the type bits clear.
        write_data_dword(&mut pci, 0xfeed_0000);
        assert_eq!(read_data_dword(&mut pci), 0xfeed_0000);
    }

    #[test]
    fn bar_sizing_io() {
        let mut pci = PciStub::new();
        // 256-byte I/O BAR at BAR1.
        let dev = ConfigSpace::device(0x1af4, 0x1000, 0x02_00_00, 0x00).with_bar(1, Bar::io(0x100));
        pci.add_function(0, 6, 0, dev);

        let bar1_off = (BAR0_OFFSET + 4) as u8;
        // Type bits: I/O BAR has bit0 set.
        set_address(&mut pci, make_address(0, 6, 0, bar1_off));
        assert_eq!(read_data_dword(&mut pci), 0x0000_0001);

        // Probe for size.
        write_data_dword(&mut pci, 0xffff_ffff);
        let mask = read_data_dword(&mut pci);
        // 256-byte region: address bits above bit 8 are ones, bit 0 = 1 (I/O),
        // bits [7:1] within the size are zero.
        assert_eq!(mask, 0xffff_ff01);
    }

    #[test]
    fn bar_sizing_mem64_prefetch() {
        let mut pci = PciStub::new();
        // 1 MiB prefetchable 64-bit memory BAR at BAR2.
        let dev = ConfigSpace::device(0x1af4, 0x1000, 0x02_00_00, 0x00)
            .with_bar(2, Bar::mem64(0x10_0000));
        pci.add_function(0, 7, 0, dev);

        let bar2_off = (BAR0_OFFSET + 8) as u8;
        set_address(&mut pci, make_address(0, 7, 0, bar2_off));
        // Type bits: bit0=0 (mem), bits[2:1]=10b (64-bit) => 0b100.
        assert_eq!(read_data_dword(&mut pci) & 0xf, 0b100);

        write_data_dword(&mut pci, 0xffff_ffff);
        let mask = read_data_dword(&mut pci);
        // 1 MiB: low 20 bits within the size are zero except the 64-bit type
        // bit (0b100); upper bits are ones.
        assert_eq!(mask, 0xfff0_0000 | 0b100);
    }

    #[test]
    fn config_space_helpers_roundtrip() {
        let mut cs = ConfigSpace::new();
        cs.set_u8(0x10, 0x12);
        cs.set_u16(0x20, 0xbeef);
        cs.set_u32(0x30, 0xdead_beef);
        assert_eq!(cs.bytes[0x10], 0x12);
        assert_eq!(cs.get_u32(0x20) & 0xffff, 0xbeef);
        assert_eq!(cs.get_u32(0x30), 0xdead_beef);
    }
}
