//! Memory Management Unit - page table translation with TLB caching.

use std::cell::RefCell;
use std::sync::{Arc, Mutex};

use crate::devices::pci::PciStub;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap};

use crate::cpu::SystemRegisters;
use crate::error::{Error, Result};

#[cfg(feature = "profiling")]
use crate::profiling;

use super::timing;

// LAPIC constants
const LAPIC_BASE: u64 = 0xFEE00000;
const LAPIC_SIZE: u64 = 0x1000;
const LAPIC_ID: u64 = 0x020;
const LAPIC_VERSION: u64 = 0x030;
const LAPIC_TPR: u64 = 0x080;
const LAPIC_EOI: u64 = 0x0B0;
const LAPIC_LDR: u64 = 0x0D0;
const LAPIC_DFR: u64 = 0x0E0;
const LAPIC_SVR: u64 = 0x0F0;
const LAPIC_ISR_BASE: u64 = 0x100;
const LAPIC_TMR_BASE: u64 = 0x180;
const LAPIC_IRR_BASE: u64 = 0x200;
const LAPIC_ESR: u64 = 0x280;
const LAPIC_ICR_LOW: u64 = 0x300;
const LAPIC_ICR_HIGH: u64 = 0x310;
const LAPIC_LVT_TIMER: u64 = 0x320;
const LAPIC_LVT_LINT0: u64 = 0x350;
const LAPIC_LVT_LINT1: u64 = 0x360;
const LAPIC_LVT_ERROR: u64 = 0x370;
const LAPIC_TIMER_ICR: u64 = 0x380;
const LAPIC_TIMER_CCR: u64 = 0x390;
const LAPIC_TIMER_DCR: u64 = 0x3E0;

const LVT_MASK: u32 = 1 << 16;
const SVR_APIC_ENABLED: u32 = 1 << 8;

/// Inline LAPIC state for emulator
#[derive(Clone)]
struct InlineLapic {
    id: u32,
    version: u32,
    tpr: u32,
    ldr: u32,
    dfr: u32,
    svr: u32,
    isr: [u32; 8],
    tmr: [u32; 8],
    irr: [u32; 8],
    esr: u32,
    icr: u64,
    lvt_timer: u32,
    lvt_lint0: u32,
    lvt_lint1: u32,
    lvt_error: u32,
    timer_initial_count: u32,
    timer_divide_config: u32,
    /// Nanoseconds elapsed since emulator start when timer was started (None = timer not running)
    timer_start_nanos: Option<u64>,
    /// Pending timer interrupt vector (not yet delivered)
    pending_timer_vector: Option<u8>,
}

impl InlineLapic {
    fn new() -> Self {
        InlineLapic {
            id: 0,
            version: 0x00050014, // Modern APIC
            tpr: 0,
            ldr: 0,
            dfr: 0xFFFFFFFF,
            svr: 0x1FF, // APIC enabled (bit 8), spurious vector 0xFF - Virtual Wire Mode
            isr: [0; 8],
            tmr: [0; 8],
            irr: [0; 8],
            esr: 0,
            icr: 0,
            lvt_timer: LVT_MASK,
            lvt_lint0: 0x700, // ExtInt mode, not masked - Virtual Wire Mode
            lvt_lint1: 0x400, // NMI mode, not masked
            lvt_error: LVT_MASK,
            timer_initial_count: 0,
            timer_divide_config: 0,
            timer_start_nanos: None,
            pending_timer_vector: None,
        }
    }

    fn timer_divisor(&self) -> u32 {
        let bits = (self.timer_divide_config & 0x3) | ((self.timer_divide_config >> 1) & 0x4);
        match bits {
            0b000 => 2,
            0b001 => 4,
            0b010 => 8,
            0b011 => 16,
            0b100 => 32,
            0b101 => 64,
            0b110 => 128,
            0b111 => 1,
            _ => 1,
        }
    }

    fn read(&self, offset: u64) -> u32 {
        match offset {
            LAPIC_ID => self.id,
            LAPIC_VERSION => self.version,
            LAPIC_TPR => self.tpr,
            LAPIC_LDR => self.ldr,
            LAPIC_DFR => self.dfr,
            LAPIC_SVR => self.svr,
            o if o >= LAPIC_ISR_BASE && o < LAPIC_ISR_BASE + 0x80 => {
                let idx = ((o - LAPIC_ISR_BASE) / 0x10) as usize;
                if idx < 8 { self.isr[idx] } else { 0 }
            }
            o if o >= LAPIC_TMR_BASE && o < LAPIC_TMR_BASE + 0x80 => {
                let idx = ((o - LAPIC_TMR_BASE) / 0x10) as usize;
                if idx < 8 { self.tmr[idx] } else { 0 }
            }
            o if o >= LAPIC_IRR_BASE && o < LAPIC_IRR_BASE + 0x80 => {
                let idx = ((o - LAPIC_IRR_BASE) / 0x10) as usize;
                if idx < 8 { self.irr[idx] } else { 0 }
            }
            LAPIC_ESR => self.esr,
            LAPIC_ICR_LOW => self.icr as u32,
            LAPIC_ICR_HIGH => (self.icr >> 32) as u32,
            LAPIC_LVT_TIMER => self.lvt_timer,
            LAPIC_LVT_LINT0 => self.lvt_lint0,
            LAPIC_LVT_LINT1 => self.lvt_lint1,
            LAPIC_LVT_ERROR => self.lvt_error,
            LAPIC_TIMER_ICR => self.timer_initial_count,
            LAPIC_TIMER_CCR => {
                // Compute current count based on wall-clock time
                if self.timer_initial_count == 0 || self.timer_start_nanos.is_none() {
                    return 0;
                }
                let start_nanos = self.timer_start_nanos.unwrap();
                let current_nanos = timing::elapsed_nanos();
                let elapsed_nanos = current_nanos.saturating_sub(start_nanos);

                // Convert nanoseconds to timer ticks:
                // LAPIC timer base frequency is 1 GHz, so 1 tick = 1 nanosecond at divisor 1
                // With divisor, timer tick rate = 1 GHz / divisor
                // Timer ticks elapsed = elapsed_nanos / divisor
                let divisor = self.timer_divisor() as u64;
                let elapsed_ticks = elapsed_nanos / divisor;

                let initial = self.timer_initial_count as u64;
                let mode = (self.lvt_timer >> 17) & 0x3;

                match mode {
                    0 => {
                        // One-shot: count down to 0
                        if elapsed_ticks >= initial {
                            0
                        } else {
                            (initial - elapsed_ticks) as u32
                        }
                    }
                    1 => {
                        // Periodic: wrap around
                        if initial == 0 {
                            0
                        } else {
                            let remainder = elapsed_ticks % initial;
                            (initial - remainder) as u32
                        }
                    }
                    _ => 0, // TSC-deadline mode returns 0
                }
            }
            LAPIC_TIMER_DCR => self.timer_divide_config,
            _ => 0,
        }
    }

    fn write(&mut self, offset: u64, value: u32) {
        match offset {
            LAPIC_ID => self.id = value & 0xFF000000,
            LAPIC_TPR => self.tpr = value & 0xFF,
            LAPIC_EOI => {
                // Clear highest priority in-service bit
                for i in (0..8).rev() {
                    if self.isr[i] != 0 {
                        for bit in (0..32).rev() {
                            if self.isr[i] & (1 << bit) != 0 {
                                self.isr[i] &= !(1 << bit);
                                return;
                            }
                        }
                    }
                }
            }
            LAPIC_LDR => self.ldr = value,
            LAPIC_DFR => self.dfr = value,
            LAPIC_SVR => self.svr = value,
            LAPIC_ESR => self.esr = 0,
            LAPIC_ICR_LOW => self.icr = (self.icr & 0xFFFFFFFF00000000) | (value as u64),
            LAPIC_ICR_HIGH => self.icr = (self.icr & 0x00000000FFFFFFFF) | ((value as u64) << 32),
            LAPIC_LVT_TIMER => {
                tracing::debug!(
                    "LAPIC LVT_TIMER write: value={:#x}, vector={}, mode={}, masked={}",
                    value,
                    value & 0xFF,
                    (value >> 17) & 3,
                    (value & 0x10000) != 0
                );
                self.lvt_timer = value;
            }
            LAPIC_LVT_LINT0 => self.lvt_lint0 = value,
            LAPIC_LVT_LINT1 => self.lvt_lint1 = value,
            LAPIC_LVT_ERROR => self.lvt_error = value,
            LAPIC_TIMER_ICR => {
                eprintln!(
                    "[LAPIC] TIMER_ICR write: count={:#x}, lvt_timer={:#x}, divisor={}, masked={}",
                    value,
                    self.lvt_timer,
                    self.timer_divisor(),
                    (self.lvt_timer & LVT_MASK) != 0
                );
                self.timer_initial_count = value;
                if value > 0 {
                    self.timer_start_nanos = Some(timing::elapsed_nanos());
                    self.pending_timer_vector = None; // Clear any pending interrupt on restart
                } else {
                    self.timer_start_nanos = None;
                }
            }
            LAPIC_TIMER_DCR => self.timer_divide_config = value & 0xB,
            _ => {}
        }
    }

    fn is_enabled(&self) -> bool {
        (self.svr & SVR_APIC_ENABLED) != 0
    }

    /// Check if there's a pending interrupt that hasn't been delivered yet
    fn has_pending(&self) -> bool {
        self.pending_timer_vector.is_some()
    }

    /// Get pending interrupt vector without clearing it
    fn get_pending(&self) -> Option<u8> {
        self.pending_timer_vector
    }

    /// Clear the pending interrupt (called after successful injection)
    fn clear_pending(&mut self) {
        self.pending_timer_vector = None;
    }

    /// Check if timer interrupt should fire and return vector if so.
    /// If an interrupt fires but can't be delivered, it becomes pending.
    fn tick_timer(&mut self) -> Option<u8> {
        // If there's already a pending interrupt, return it
        if let Some(vector) = self.pending_timer_vector {
            return Some(vector);
        }

        if !self.is_enabled() || self.timer_initial_count == 0 {
            return None;
        }
        // Check if timer is masked
        if (self.lvt_timer & LVT_MASK) != 0 {
            return None;
        }
        let Some(start_nanos) = self.timer_start_nanos else {
            return None;
        };

        let current_nanos = timing::elapsed_nanos();
        let elapsed_nanos = current_nanos.saturating_sub(start_nanos);

        // Convert nanoseconds to timer ticks (same formula as in read)
        // LAPIC timer base frequency is 1 GHz, so 1 tick = 1 nanosecond at divisor 1
        let divisor = self.timer_divisor() as u64;
        let elapsed_ticks = elapsed_nanos / divisor;
        let initial = self.timer_initial_count as u64;

        let mode = (self.lvt_timer >> 17) & 0x3; // Timer mode bits
        let vector = (self.lvt_timer & 0xFF) as u8;

        match mode {
            0 => {
                // One-shot: fire once when count reaches 0
                if elapsed_ticks >= initial {
                    self.timer_start_nanos = None; // Stop timer
                    self.pending_timer_vector = Some(vector);
                    return Some(vector);
                }
            }
            1 => {
                // Periodic: fire and restart
                if elapsed_ticks >= initial {
                    // Restart timer from current time
                    self.timer_start_nanos = Some(current_nanos);
                    self.pending_timer_vector = Some(vector);
                    return Some(vector);
                }
            }
            _ => {}
        }
        None
    }
}

/// Page table entry flags.
#[allow(dead_code)]
mod flags {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER: u64 = 1 << 2;
    pub const WRITE_THROUGH: u64 = 1 << 3;
    pub const CACHE_DISABLE: u64 = 1 << 4;
    pub const ACCESSED: u64 = 1 << 5;
    pub const DIRTY: u64 = 1 << 6;
    pub const HUGE_PAGE: u64 = 1 << 7;
    pub const GLOBAL: u64 = 1 << 8;
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// Control register bits.
mod cr0 {
    pub const PE: u64 = 1 << 0; // Protected Mode Enable
    pub const PG: u64 = 1 << 31; // Paging Enable
}

/// Physical address bits (must match CPUID 0x80000008 EAX[7:0])
const PHYS_BITS: u32 = 48;
/// Reserved bits mask: bits in PTE address field that must be zero
/// For 40 phys bits: bits 51:40 in the address portion (0x000FFF0000000000)
const PTE_RSVD_MASK: u64 = !((1u64 << PHYS_BITS) - 1) & 0x000F_FFFF_FFFF_F000;

#[allow(dead_code)]
mod cr4 {
    pub const PAE: u64 = 1 << 5; // Physical Address Extension
}

mod efer {
    pub const LME: u64 = 1 << 8; // Long Mode Enable
    pub const LMA: u64 = 1 << 10; // Long Mode Active
}

/// Linux kernel direct map virtual address base (physical memory mapped here)
const DIRECT_MAP_BASE: u64 = 0xffff888000000000;
/// Direct map covers up to 64TB of physical memory
const DIRECT_MAP_END: u64 = 0xffffc87fffffffff;

/// Memory access type.
#[derive(Debug, Clone, Copy)]
pub enum AccessType {
    Read,
    Write,
    Execute,
}

/// TLB entry - caches virtual to physical page translations.
#[derive(Clone, Copy)]
struct TlbEntry {
    /// Virtual page number (vaddr >> 12), with CR3 mixed in for context tagging
    tag: u64,
    /// Physical page base address (already shifted)
    phys_base: u64,
    /// Page size shift: 12 for 4KB, 21 for 2MB, 30 for 1GB
    page_shift: u8,
    /// Valid bit
    valid: bool,
}

impl Default for TlbEntry {
    #[inline(always)]
    fn default() -> Self {
        TlbEntry {
            tag: 0,
            phys_base: 0,
            page_shift: 12,
            valid: false,
        }
    }
}

/// TLB size - must be power of 2 for fast indexing
const TLB_SIZE: usize = 256;
const TLB_MASK: usize = TLB_SIZE - 1;

/// Code page bitmap size - tracks which pages have been executed from.
/// Each bit represents a 4KB page. 64KB of bitmap = 512MB of tracked memory.
/// For larger address spaces, we use a hash of the virtual address.
const CODE_PAGE_BITMAP_SIZE: usize = 64 * 1024; // 64KB = 512K pages = 2GB coverage
const CODE_PAGE_BITMAP_MASK: usize = CODE_PAGE_BITMAP_SIZE * 8 - 1;

/// Memory Management Unit for address translation with TLB.
pub struct Mmu {
    memory: Arc<GuestMemoryMmap>,
    /// Direct-mapped TLB for 4KB pages
    tlb: [TlbEntry; TLB_SIZE],
    /// Cached CR3 for detecting context switches
    cached_cr3: u64,
    /// Bitmap tracking pages that have been executed from (code pages).
    /// Used for self-modifying code detection: writes to code pages
    /// require decode cache invalidation.
    code_page_bitmap: Box<[u8; CODE_PAGE_BITMAP_SIZE]>,
    /// Self-modifying-code journal: page bases (virtual) of code pages written
    /// since the vcpu last drained it. EVERY guest write funnels through the
    /// `write_u*` methods, which record code-page writes here; the vcpu drains
    /// this at each instruction boundary (`step`) and invalidates the decode +
    /// JIT caches for those pages. This is the single, complete SMC choke point
    /// (the old per-handler `check_smc` missed the ~39 sites that call
    /// `write_u*` directly).
    smc_dirty_pages: Vec<u64>,
    /// Inline LAPIC for emulator (handles MMIO to 0xFEE00000)
    /// Uses RefCell for interior mutability since read_phys/write_phys take &self
    lapic: RefCell<InlineLapic>,
    /// Cached host base pointer + length of the contiguous RAM region at GPA 0.
    /// Lets read_phys/write_phys serve in-RAM accesses with a direct pointer copy,
    /// skipping vm-memory's per-access region search + bounds-checked volatile copy.
    /// Stored as usize (not a raw pointer) so Mmu remains Send.
    ram_host_base: usize,
    ram_len: u64,
    /// Upper bound (exclusive) of the RAM window that contains no MMIO hole, so
    /// any access fully below it is plain RAM. Equals `ram_len` when the LAPIC
    /// window sits entirely above RAM (the common small-guest case), else it is
    /// capped at `LAPIC_BASE`. Used by `ram_ptr` to elide the per-access MMIO
    /// overlap test on the hot path.
    ram_fast_len: u64,
    /// Optional PCI host bridge for routing BAR-mapped MMIO. `None` (the
    /// default) keeps the memory path byte-identical to a PCI-less machine.
    pci: Option<Arc<Mutex<PciStub>>>,
    /// Physical aperture [pci_ap_lo, pci_ap_hi) diverted from RAM to PCI BAR
    /// dispatch. Empty (lo=u64::MAX, hi=0) when no PCI devices are attached, so
    /// every aperture test is trivially false and the hot path is unaffected.
    pci_ap_lo: u64,
    pci_ap_hi: u64,
}

impl Mmu {
    pub fn new(memory: Arc<GuestMemoryMmap>) -> Self {
        // Cache the host pointer + length of the contiguous RAM region mapped at
        // guest-physical 0 (rax allocates guest RAM as a single such region).
        let (ram_host_base, ram_len) = {
            use vm_memory::{Address, GuestMemory, GuestMemoryRegion};
            memory
                .iter()
                .find(|r| r.start_addr().raw_value() == 0)
                .map(|r| (r.as_ptr() as usize, r.len()))
                .unwrap_or((0, 0))
        };
        // If RAM ends at or below the LAPIC window, no in-RAM access can ever
        // overlap MMIO, so the whole region is "fast". Otherwise the region
        // straddles the LAPIC hole; only accesses strictly below LAPIC_BASE are
        // guaranteed MMIO-free and take the single-compare fast path.
        let ram_fast_len = if ram_len <= LAPIC_BASE {
            ram_len
        } else {
            LAPIC_BASE
        };
        Mmu {
            memory,
            tlb: [TlbEntry::default(); TLB_SIZE],
            cached_cr3: 0,
            code_page_bitmap: Box::new([0u8; CODE_PAGE_BITMAP_SIZE]),
            smc_dirty_pages: Vec::new(),
            lapic: RefCell::new(InlineLapic::new()),
            ram_host_base,
            ram_len,
            ram_fast_len,
            pci: None,
            pci_ap_lo: u64::MAX,
            pci_ap_hi: 0,
        }
    }

    /// Attach the PCI host bridge and divert the physical aperture
    /// `[ap_base, ap_end)` from RAM to PCI BAR dispatch. Lowering `ram_fast_len`
    /// to `ap_base` pushes aperture accesses onto the cold path (`ram_ptr_high`
    /// / `read_phys`) where they are routed to the bridge; all RAM below the
    /// aperture (including the per-CPU overflow window) stays on the fast path.
    pub fn set_pci_bridge(&mut self, bridge: Arc<Mutex<PciStub>>, ap_base: u64, ap_end: u64) {
        self.pci = Some(bridge);
        self.pci_ap_lo = ap_base;
        self.pci_ap_hi = ap_end;
        if ap_base < self.ram_fast_len {
            self.ram_fast_len = ap_base;
        }
    }

    /// True when `paddr` falls in the PCI MMIO aperture. Trivially false (one
    /// compare against `u64::MAX`) when no PCI devices are attached.
    #[inline(always)]
    fn in_pci_aperture(&self, paddr: u64) -> bool {
        paddr >= self.pci_ap_lo && paddr < self.pci_ap_hi
    }

    /// True when `[paddr, paddr+len)` lies entirely within the cached RAM region.
    #[inline(always)]
    fn in_ram(&self, paddr: u64, len: usize) -> bool {
        self.ram_len != 0
            && paddr
                .checked_add(len as u64)
                .map_or(false, |end| end <= self.ram_len)
    }

    /// Host pointer for `[paddr, paddr+LEN)` when it lies wholly within RAM and
    /// outside the LAPIC MMIO window, else `None`.
    ///
    /// Common case is a single comparison: `paddr <= ram_fast_len - LEN`, where
    /// `ram_fast_len` bounds the MMIO-free RAM window, so a pass guarantees the
    /// access is plain RAM. `LEN` is a const so the typed accessors inline to a
    /// bounds-test + pointer read/write. The cold branch handles guests whose
    /// RAM extends past the LAPIC window (access at/above `ram_fast_len`).
    #[inline(always)]
    fn ram_ptr<const LEN: u64>(&self, paddr: u64) -> Option<usize> {
        // `checked_sub` yields None when ram_fast_len < LEN (zero/tiny RAM), so an
        // empty or sub-width RAM window can never spuriously pass the bound and
        // hand out a near-null host pointer — the unsafe accessors below rely on
        // this rather than on a non-local invariant (e.g. translate() failing
        // first). For real RAM this is a single compare on the hot path.
        match self.ram_fast_len.checked_sub(LEN) {
            Some(bound) if paddr <= bound => Some(self.ram_host_base + paddr as usize),
            _ => self.ram_ptr_high::<LEN>(paddr),
        }
    }

    /// Cold fallback of `ram_ptr` for accesses at or above the fast window:
    /// either above the LAPIC hole (still RAM) or genuinely out of RAM/in MMIO.
    #[cold]
    #[inline(never)]
    fn ram_ptr_high<const LEN: u64>(&self, paddr: u64) -> Option<usize> {
        // Aperture addresses are not RAM: returning None routes the typed
        // accessor through read_phys/write_phys, which dispatch to the bridge.
        if self.in_pci_aperture(paddr) {
            return None;
        }
        match self.ram_len.checked_sub(LEN) {
            Some(bound)
                if paddr <= bound
                    && !(paddr < LAPIC_BASE + LAPIC_SIZE
                        && paddr.wrapping_add(LEN) > LAPIC_BASE) =>
            {
                Some(self.ram_host_base + paddr as usize)
            }
            _ => None,
        }
    }

    /// Get the size of guest memory in bytes
    pub fn memory_size(&self) -> u64 {
        use vm_memory::{Address, GuestMemory};
        self.memory.last_addr().raw_value() + 1
    }

    /// Check if an address is in the LAPIC MMIO range
    #[inline(always)]
    fn is_lapic_addr(paddr: u64) -> bool {
        paddr >= LAPIC_BASE && paddr < LAPIC_BASE + LAPIC_SIZE
    }

    /// Tick the inline LAPIC timer and return pending interrupt vector if any
    pub fn tick_lapic_timer(&self) -> Option<u8> {
        self.lapic.borrow_mut().tick_timer()
    }

    /// Clear pending LAPIC timer interrupt (call after successful injection)
    pub fn clear_lapic_pending(&self) {
        self.lapic.borrow_mut().clear_pending()
    }

    /// Check if there's a pending LAPIC timer interrupt
    pub fn has_lapic_pending(&self) -> bool {
        self.lapic.borrow().has_pending()
    }

    // =========================================================================
    // Self-modifying code detection
    // =========================================================================

    /// Compute the bitmap index for a virtual page.
    #[inline(always)]
    fn code_page_index(vaddr: u64) -> (usize, u8) {
        // Use page number (vaddr >> 12) and hash to fit in bitmap
        let page_num = (vaddr >> 12) as usize;
        let bit_index = page_num & CODE_PAGE_BITMAP_MASK;
        let byte_index = bit_index >> 3;
        let bit_offset = (bit_index & 7) as u8;
        (byte_index, 1u8 << bit_offset)
    }

    /// Mark a page as containing executed code.
    /// Called when fetching instructions from a page.
    #[inline(always)]
    pub fn mark_code_page(&mut self, vaddr: u64) {
        let (byte_idx, bit_mask) = Self::code_page_index(vaddr);
        self.code_page_bitmap[byte_idx] |= bit_mask;
    }

    /// Check if a page has been marked as code.
    /// Called when writing to memory to detect self-modifying code.
    #[inline(always)]
    pub fn is_code_page(&self, vaddr: u64) -> bool {
        let (byte_idx, bit_mask) = Self::code_page_index(vaddr);
        (self.code_page_bitmap[byte_idx] & bit_mask) != 0
    }

    /// Record that `vaddr` was written, if it lies on a known code page (SMC).
    /// Called from every `write_u*` entry point so no store can bypass SMC
    /// detection. The vcpu drains [`Self::take_smc_dirty`] each instruction.
    #[inline(always)]
    fn note_smc(&mut self, vaddr: u64) {
        if self.is_code_page(vaddr) {
            let page = vaddr & !0xFFF;
            if !self.smc_dirty_pages.contains(&page) {
                self.smc_dirty_pages.push(page);
            }
        }
    }

    /// True if any code page has been written since the last drain (cheap guard
    /// for the per-instruction hot path).
    #[inline(always)]
    pub fn has_smc_dirty(&self) -> bool {
        !self.smc_dirty_pages.is_empty()
    }

    /// Take and clear the set of code-page bases written since the last drain.
    pub fn take_smc_dirty(&mut self) -> Vec<u64> {
        std::mem::take(&mut self.smc_dirty_pages)
    }

    /// Clear the code page bitmap.
    /// Should be called on context switch or when JIT cache is fully cleared.
    #[inline]
    pub fn clear_code_pages(&mut self) {
        self.code_page_bitmap.fill(0);
    }

    /// Check if paging is enabled.
    #[inline(always)]
    fn paging_enabled(&self, sregs: &SystemRegisters) -> bool {
        (sregs.cr0 & cr0::PG) != 0 && (sregs.cr0 & cr0::PE) != 0
    }

    /// Check if we're in 64-bit long mode.
    #[inline(always)]
    fn long_mode(&self, sregs: &SystemRegisters) -> bool {
        (sregs.efer & efer::LMA) != 0 && (sregs.efer & efer::LME) != 0
    }

    /// Compute TLB index from virtual address (uses bits that vary most)
    #[inline(always)]
    fn tlb_index(vaddr: u64) -> usize {
        // Use bits 12-19 (page number bits) for index
        ((vaddr >> 12) as usize) & TLB_MASK
    }

    /// Compute TLB tag from virtual address and CR3
    #[inline(always)]
    fn tlb_tag(vaddr: u64, cr3: u64) -> u64 {
        // Tag includes page number and CR3 to handle context switches
        // For 4KB pages: tag = VPN (bits 12-47) | (CR3 bits 12-35 shifted)
        let vpn = vaddr >> 12;
        let cr3_tag = (cr3 >> 12) & 0xFFFFFF; // 24 bits of CR3
        vpn ^ (cr3_tag << 36) // Mix CR3 into upper bits
    }

    /// Flush entire TLB (called on CR3 change)
    #[inline]
    pub fn flush_tlb(&mut self) {
        for entry in &mut self.tlb {
            entry.valid = false;
        }
    }

    /// Invalidate TLB entry for a virtual address.
    #[inline]
    pub fn invlpg(&mut self, vaddr: u64) {
        let index = Self::tlb_index(vaddr);
        self.tlb[index].valid = false;
    }

    /// Translate a virtual address to a physical address.
    #[inline]
    pub fn translate(
        &mut self,
        vaddr: u64,
        access: AccessType,
        sregs: &SystemRegisters,
    ) -> Result<u64> {
        // If paging is disabled, virtual = physical
        if !self.paging_enabled(sregs) {
            return Ok(vaddr);
        }

        // Check for CR3 change (context switch) - flush TLB
        if sregs.cr3 != self.cached_cr3 {
            self.flush_tlb();
            self.cached_cr3 = sregs.cr3;
        }

        // TLB lookup - only use for reads (writes need permission check)
        // TODO: Could cache write permission in TLB for better performance
        if !matches!(access, AccessType::Write) {
            let index = Self::tlb_index(vaddr);
            let tag = Self::tlb_tag(vaddr, sregs.cr3);

            let entry = &self.tlb[index];
            if entry.valid && entry.tag == tag {
                // TLB hit! Fast path
                let offset_mask = (1u64 << entry.page_shift) - 1;
                let paddr = entry.phys_base | (vaddr & offset_mask);
                return Ok(paddr);
            }
        }

        // TLB miss or write - do full page table walk with permission check
        let index = Self::tlb_index(vaddr);
        let tag = Self::tlb_tag(vaddr, sregs.cr3);

        // Try normal page table walk first
        match self.translate_slow(vaddr, access, sregs, index, tag) {
            Ok(paddr) => Ok(paddr),
            Err(Error::PageFault { vaddr, error_code }) => {
                // Page table walk failed. Check if this is a direct map access
                // that falls outside the kernel's page tables but within our
                // actual physical memory allocation (used for per-CPU overflow).
                self.try_direct_map_fallback(vaddr, error_code)
            }
            Err(e) => Err(e),
        }
    }

    /// Fallback translation for direct map addresses outside kernel's page tables.
    ///
    /// The kernel only creates page table entries for memory reported in e820,
    /// but we allocate extra memory for per-CPU overflow. When the kernel tries
    /// to access per-CPU data at addresses just past the e820 range, the page
    /// table walk fails. This function handles those accesses by directly
    /// computing the physical address from the direct map virtual address.
    fn try_direct_map_fallback(&self, vaddr: u64, error_code: u64) -> Result<u64> {
        // Check if address is in the direct map region
        if vaddr >= DIRECT_MAP_BASE && vaddr <= DIRECT_MAP_END {
            let paddr = vaddr - DIRECT_MAP_BASE;

            // Check if physical address is within our actual allocation
            let mem_size = self.memory_size();
            if paddr < mem_size {
                // Allow access to memory we've allocated but kernel doesn't know about
                return Ok(paddr);
            }
        }

        // Not in direct map or outside our allocation - return the original page fault
        // with the error_code computed by translate_slow (includes U/S and W/R bits)
        Err(Error::PageFault { vaddr, error_code })
    }

    /// Slow path: full page table walk (called on TLB miss)
    #[cold]
    fn translate_slow(
        &mut self,
        vaddr: u64,
        access: AccessType,
        sregs: &SystemRegisters,
        tlb_index: usize,
        tlb_tag: u64,
    ) -> Result<u64> {
        // For now, we only support 64-bit 4-level paging
        if !self.long_mode(sregs) {
            return Err(Error::Emulator(
                "only 64-bit long mode paging is supported".to_string(),
            ));
        }

        let is_write = matches!(access, AccessType::Write);

        // Determine current privilege level (CPL) from CS selector
        let cpl = (sregs.cs.selector & 0x3) as u8;
        let is_user = cpl == 3;
        // Page fault error code bits:
        // Bit 0 (P): 0 = non-present page, 1 = protection violation
        // Bit 1 (W/R): 0 = read, 1 = write
        // Bit 2 (U/S): 0 = supervisor, 1 = user
        let user_bit = if is_user { 0x4 } else { 0 };
        let write_bit = if is_write { 0x2 } else { 0 };

        // 4-level paging: PML4 -> PDPT -> PD -> PT
        let pml4_base = sregs.cr3 & !0xFFF;
        let pml4_index = (vaddr >> 39) & 0x1FF;
        let pdpt_index = (vaddr >> 30) & 0x1FF;
        let pd_index = (vaddr >> 21) & 0x1FF;
        let pt_index = (vaddr >> 12) & 0x1FF;

        // Read PML4 entry
        let pml4e = self.read_pte(pml4_base + pml4_index * 8)?;
        if pml4e & flags::PRESENT == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit,
            });
        }
        // Check reserved bits (must be zero) - error code 0x9 = RSVD | P
        if pml4e & PTE_RSVD_MASK != 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x9,
            });
        }
        // Check U/S permission: if user mode, page must have USER bit set
        if is_user && pml4e & flags::USER == 0 {
            // User mode accessing supervisor page = protection violation
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x1,
            });
        }
        // Check write permission at PML4 level
        if is_write && pml4e & flags::WRITABLE == 0 {
            // error_code bit 1 = write access, bit 0 = present
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | 0x3,
            });
        }

        // Read PDPT entry
        let pdpt_base = pml4e & 0x000F_FFFF_FFFF_F000;
        let pdpte = self.read_pte(pdpt_base + pdpt_index * 8)?;
        if pdpte & flags::PRESENT == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit,
            });
        }
        // Check reserved bits
        if pdpte & PTE_RSVD_MASK != 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x9,
            });
        }
        // Check U/S permission
        if is_user && pdpte & flags::USER == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x1,
            });
        }
        // Check write permission at PDPT level
        if is_write && pdpte & flags::WRITABLE == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | 0x3,
            });
        }

        // Check for 1GB huge page
        if pdpte & flags::HUGE_PAGE != 0 {
            // For 1GB pages, bits 29:13 in address portion must be zero (bit 12 is PAT)
            // Additional reserved mask: (0x40000000 - 1) & 0x000FFFFFFFFFF000 & ~0x1000 = 0x3FFFE000
            let huge_rsvd = 0x3FFFE000u64;
            if pdpte & (PTE_RSVD_MASK | huge_rsvd) != 0 {
                return Err(Error::PageFault {
                    vaddr,
                    error_code: user_bit | write_bit | 0x9,
                });
            }
            let page_base = pdpte & 0x000F_FFFF_C000_0000;
            // Cache in TLB
            self.tlb[tlb_index] = TlbEntry {
                tag: tlb_tag,
                phys_base: page_base,
                page_shift: 30, // 1GB
                valid: true,
            };
            return Ok(page_base | (vaddr & 0x3FFF_FFFF));
        }

        // Read PD entry
        let pd_base = pdpte & 0x000F_FFFF_FFFF_F000;
        let pde = self.read_pte(pd_base + pd_index * 8)?;
        if pde & flags::PRESENT == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit,
            });
        }
        // Check reserved bits
        if pde & PTE_RSVD_MASK != 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x9,
            });
        }
        // Check U/S permission
        if is_user && pde & flags::USER == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x1,
            });
        }
        // Check write permission at PD level
        if is_write && pde & flags::WRITABLE == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | 0x3,
            });
        }

        // Check for 2MB huge page
        if pde & flags::HUGE_PAGE != 0 {
            // For 2MB pages, bits 20:13 in address portion must be zero (bit 12 is PAT)
            // Additional reserved mask: (0x200000 - 1) & 0x000FFFFFFFFFF000 & ~0x1000 = 0x1FE000
            let huge_rsvd = 0x1FE000u64;
            if pde & (PTE_RSVD_MASK | huge_rsvd) != 0 {
                return Err(Error::PageFault {
                    vaddr,
                    error_code: user_bit | write_bit | 0x9,
                });
            }
            let page_base = pde & 0x000F_FFFF_FFE0_0000;
            let paddr = page_base | (vaddr & 0x1F_FFFF);

            // Cache in TLB
            self.tlb[tlb_index] = TlbEntry {
                tag: tlb_tag,
                phys_base: page_base,
                page_shift: 21, // 2MB
                valid: true,
            };
            return Ok(paddr);
        }

        // Read PT entry
        let pt_base = pde & 0x000F_FFFF_FFFF_F000;
        let pt_addr = pt_base + pt_index * 8;
        let pte = self.read_pte(pt_addr)?;
        if pte & flags::PRESENT == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit,
            });
        }
        // Check reserved bits
        if pte & PTE_RSVD_MASK != 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x9,
            });
        }
        // Check U/S permission
        if is_user && pte & flags::USER == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | write_bit | 0x1,
            });
        }
        // Check write permission at PT level
        if is_write && pte & flags::WRITABLE == 0 {
            return Err(Error::PageFault {
                vaddr,
                error_code: user_bit | 0x3,
            });
        }

        let page_base = pte & 0x000F_FFFF_FFFF_F000;
        let offset = vaddr & 0xFFF;
        let paddr = page_base | offset;

        // Cache in TLB
        self.tlb[tlb_index] = TlbEntry {
            tag: tlb_tag,
            phys_base: page_base,
            page_shift: 12, // 4KB
            valid: true,
        };

        Ok(paddr)
    }

    /// Read a page table entry from physical memory.
    #[inline(always)]
    fn read_pte(&self, paddr: u64) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.memory
            .read_slice(&mut buf, GuestAddress(paddr))
            .map_err(|e| Error::Emulator(format!("failed to read PTE at {:#x}: {}", paddr, e)))?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Read bytes from guest memory (physical address).
    #[inline(always)]
    pub fn read_phys(&self, paddr: u64, buf: &mut [u8]) -> Result<()> {
        // Handle LAPIC MMIO
        if Self::is_lapic_addr(paddr) {
            let offset = paddr - LAPIC_BASE;
            let aligned_offset = offset & !0x3;
            let value = self.lapic.borrow().read(aligned_offset);
            // LAPIC reads are always 32-bit aligned
            match buf.len() {
                1 => {
                    let byte_offset = (offset & 0x3) as usize;
                    buf[0] = ((value >> (byte_offset * 8)) & 0xFF) as u8;
                }
                2 => {
                    let byte_offset = (offset & 0x2) as usize;
                    let word = ((value >> (byte_offset * 8)) & 0xFFFF) as u16;
                    buf[0] = (word & 0xFF) as u8;
                    buf[1] = ((word >> 8) & 0xFF) as u8;
                }
                4 => {
                    buf[0] = (value & 0xFF) as u8;
                    buf[1] = ((value >> 8) & 0xFF) as u8;
                    buf[2] = ((value >> 16) & 0xFF) as u8;
                    buf[3] = ((value >> 24) & 0xFF) as u8;
                }
                8 => {
                    // 64-bit read: read two consecutive 32-bit registers
                    buf[0] = (value & 0xFF) as u8;
                    buf[1] = ((value >> 8) & 0xFF) as u8;
                    buf[2] = ((value >> 16) & 0xFF) as u8;
                    buf[3] = ((value >> 24) & 0xFF) as u8;
                    let value2 = self.lapic.borrow().read(aligned_offset + 0x10);
                    buf[4] = (value2 & 0xFF) as u8;
                    buf[5] = ((value2 >> 8) & 0xFF) as u8;
                    buf[6] = ((value2 >> 16) & 0xFF) as u8;
                    buf[7] = ((value2 >> 24) & 0xFF) as u8;
                }
                _ => buf.fill(0),
            }
            return Ok(());
        }

        // PCI MMIO aperture: route to the device whose BAR covers paddr, else
        // open-bus (0xFF). Trivially skipped when no PCI devices are attached.
        if self.in_pci_aperture(paddr) {
            if let Some(ref pci) = self.pci {
                if let Ok(mut bridge) = pci.lock() {
                    if bridge.mmio_read(paddr, buf) {
                        return Ok(());
                    }
                }
            }
            buf.fill(0xFF);
            return Ok(());
        }

        // Fast path: direct host-pointer copy for in-RAM physical addresses
        // (LAPIC MMIO was already handled above).
        if self.in_ram(paddr, buf.len()) {
            // SAFETY: [paddr, paddr+len) is within the single contiguous RAM mmap
            // region [0, ram_len); ram_host_base is that region's stable host base.
            // `buf` is a caller-owned slice that never aliases guest memory.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    (self.ram_host_base + paddr as usize) as *const u8,
                    buf.as_mut_ptr(),
                    buf.len(),
                );
            }
            return Ok(());
        }

        self.memory
            .read_slice(buf, GuestAddress(paddr))
            .map_err(|e| Error::Emulator(format!("failed to read at {:#x}: {}", paddr, e)))
    }

    /// Write bytes to guest memory (physical address).
    #[inline(always)]
    pub fn write_phys(&self, paddr: u64, buf: &[u8]) -> Result<()> {
        // Handle LAPIC MMIO
        if Self::is_lapic_addr(paddr) {
            let offset = paddr - LAPIC_BASE;
            let aligned_offset = offset & !0x3;
            let value = match buf.len() {
                1 => buf[0] as u32,
                2 => (buf[0] as u32) | ((buf[1] as u32) << 8),
                4 => {
                    (buf[0] as u32)
                        | ((buf[1] as u32) << 8)
                        | ((buf[2] as u32) << 16)
                        | ((buf[3] as u32) << 24)
                }
                8 => {
                    // 64-bit write: write two consecutive 32-bit registers
                    let lo = (buf[0] as u32)
                        | ((buf[1] as u32) << 8)
                        | ((buf[2] as u32) << 16)
                        | ((buf[3] as u32) << 24);
                    let hi = (buf[4] as u32)
                        | ((buf[5] as u32) << 8)
                        | ((buf[6] as u32) << 16)
                        | ((buf[7] as u32) << 24);
                    self.lapic.borrow_mut().write(aligned_offset, lo);
                    self.lapic.borrow_mut().write(aligned_offset + 0x10, hi);
                    return Ok(());
                }
                _ => return Ok(()),
            };
            self.lapic.borrow_mut().write(aligned_offset, value);
            return Ok(());
        }

        // PCI MMIO aperture: route to the device whose BAR covers paddr, else
        // drop the write (open bus). Trivially skipped with no PCI devices.
        if self.in_pci_aperture(paddr) {
            if let Some(ref pci) = self.pci {
                if let Ok(mut bridge) = pci.lock() {
                    bridge.mmio_write(paddr, buf);
                }
            }
            return Ok(());
        }

        // Fast path: direct host-pointer copy for in-RAM physical addresses
        // (LAPIC MMIO was already handled above).
        if self.in_ram(paddr, buf.len()) {
            // SAFETY: [paddr, paddr+len) is within the single contiguous RAM mmap
            // region [0, ram_len); ram_host_base is that region's stable host base.
            // `buf` is a caller-owned slice that never aliases guest memory.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    buf.as_ptr(),
                    (self.ram_host_base + paddr as usize) as *mut u8,
                    buf.len(),
                );
            }
            return Ok(());
        }

        self.memory
            .write_slice(buf, GuestAddress(paddr))
            .map_err(|e| Error::Emulator(format!("failed to write at {:#x}: {}", paddr, e)))
    }

    /// Read bytes from guest memory (virtual address).
    /// Fast path for single-page access, handles page crossing.
    #[inline]
    pub fn read(&mut self, vaddr: u64, buf: &mut [u8], sregs: &SystemRegisters) -> Result<()> {
        let len = buf.len();

        // Fast path: access doesn't cross page boundary
        let page_offset = (vaddr & 0xFFF) as usize;
        if page_offset + len <= 0x1000 {
            let paddr = self.translate(vaddr, AccessType::Read, sregs)?;
            return self.read_phys(paddr, buf);
        }

        // Slow path: handle page boundary crossing
        self.read_crossing(vaddr, buf, sregs)
    }

    /// Read bytes from guest memory with supervisor privilege.
    /// Used for exception/interrupt delivery where the CPU always accesses
    /// kernel data structures (IDT, TSS, etc.) as supervisor, regardless of CPL.
    #[inline]
    pub fn read_supervisor(
        &mut self,
        vaddr: u64,
        buf: &mut [u8],
        sregs: &SystemRegisters,
    ) -> Result<()> {
        // Create a temporary sregs with CPL=0 (supervisor)
        let mut supervisor_sregs = sregs.clone();
        supervisor_sregs.cs.selector &= !0x3; // Clear CPL bits to 0
        self.read(vaddr, buf, &supervisor_sregs)
    }

    /// Read a u64 from guest memory with supervisor privilege.
    pub fn read_u64_supervisor(&mut self, vaddr: u64, sregs: &SystemRegisters) -> Result<u64> {
        let mut buf = [0u8; 8];
        self.read_supervisor(vaddr, &mut buf, sregs)?;
        Ok(u64::from_le_bytes(buf))
    }

    /// Write bytes to guest memory with supervisor privilege.
    /// Used for exception/interrupt delivery.
    #[inline]
    pub fn write_supervisor(
        &mut self,
        vaddr: u64,
        buf: &[u8],
        sregs: &SystemRegisters,
    ) -> Result<()> {
        let mut supervisor_sregs = sregs.clone();
        supervisor_sregs.cs.selector &= !0x3;
        self.write(vaddr, buf, &supervisor_sregs)
    }

    /// Write a u64 to guest memory with supervisor privilege.
    pub fn write_u64_supervisor(
        &mut self,
        vaddr: u64,
        value: u64,
        sregs: &SystemRegisters,
    ) -> Result<()> {
        self.write_supervisor(vaddr, &value.to_le_bytes(), sregs)
    }

    /// Slow path for reads that cross page boundaries
    #[cold]
    fn read_crossing(&mut self, vaddr: u64, buf: &mut [u8], sregs: &SystemRegisters) -> Result<()> {
        let mut offset = 0;
        let mut remaining = buf.len();
        let mut addr = vaddr;

        while remaining > 0 {
            let paddr = self.translate(addr, AccessType::Read, sregs)?;
            let page_offset = (paddr & 0xFFF) as usize;
            let bytes_in_page = std::cmp::min(remaining, 0x1000 - page_offset);

            self.read_phys(paddr, &mut buf[offset..offset + bytes_in_page])?;

            offset += bytes_in_page;
            remaining -= bytes_in_page;
            addr += bytes_in_page as u64;
        }

        Ok(())
    }

    /// Write bytes to guest memory (virtual address).
    /// Fast path for single-page access, handles page crossing.
    #[inline]
    pub fn write(&mut self, vaddr: u64, buf: &[u8], sregs: &SystemRegisters) -> Result<()> {
        let len = buf.len();
        self.note_smc(vaddr);
        if len > 1 {
            // A multi-byte write may straddle into the next page.
            self.note_smc(vaddr.wrapping_add(len as u64 - 1));
        }

        // Fast path: access doesn't cross page boundary
        let page_offset = (vaddr & 0xFFF) as usize;
        if page_offset + len <= 0x1000 {
            let paddr = self.translate(vaddr, AccessType::Write, sregs)?;
            return self.write_phys(paddr, buf);
        }

        // Slow path: handle page boundary crossing
        self.write_crossing(vaddr, buf, sregs)
    }

    /// Slow path for writes that cross page boundaries
    #[cold]
    fn write_crossing(&mut self, vaddr: u64, buf: &[u8], sregs: &SystemRegisters) -> Result<()> {
        let mut offset = 0;
        let mut remaining = buf.len();
        let mut addr = vaddr;

        while remaining > 0 {
            let paddr = self.translate(addr, AccessType::Write, sregs)?;
            let page_offset = (paddr & 0xFFF) as usize;
            let bytes_in_page = std::cmp::min(remaining, 0x1000 - page_offset);

            self.write_phys(paddr, &buf[offset..offset + bytes_in_page])?;

            offset += bytes_in_page;
            remaining -= bytes_in_page;
            addr += bytes_in_page as u64;
        }

        Ok(())
    }

    /// Read a u8 from virtual address.
    #[inline(always)]
    pub fn read_u8(&mut self, vaddr: u64, sregs: &SystemRegisters) -> Result<u8> {
        #[cfg(feature = "profiling")]
        profiling::memory::record_read(1);

        let paddr = self.translate(vaddr, AccessType::Read, sregs)?;
        if let Some(p) = self.ram_ptr::<1>(paddr) {
            // SAFETY: ram_ptr verified [paddr, paddr+1) is in the RAM region and
            // not in MMIO; p is the corresponding host address.
            return Ok(unsafe { (p as *const u8).read() });
        }
        let mut buf = [0u8; 1];
        self.read_phys(paddr, &mut buf)?;
        Ok(buf[0])
    }

    /// Read a u16 from virtual address.
    #[inline(always)]
    pub fn read_u16(&mut self, vaddr: u64, sregs: &SystemRegisters) -> Result<u16> {
        #[cfg(feature = "profiling")]
        profiling::memory::record_read(2);

        // Fast path if not crossing page boundary
        if (vaddr & 0xFFF) <= 0xFFE {
            let paddr = self.translate(vaddr, AccessType::Read, sregs)?;
            if let Some(p) = self.ram_ptr::<2>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+2) is in-RAM, non-MMIO.
                return Ok(u16::from_le(unsafe { (p as *const u16).read_unaligned() }));
            }
            let mut buf = [0u8; 2];
            self.read_phys(paddr, &mut buf)?;
            Ok(u16::from_le_bytes(buf))
        } else {
            let mut buf = [0u8; 2];
            self.read(vaddr, &mut buf, sregs)?;
            Ok(u16::from_le_bytes(buf))
        }
    }

    /// Read a u32 from virtual address.
    #[inline(always)]
    pub fn read_u32(&mut self, vaddr: u64, sregs: &SystemRegisters) -> Result<u32> {
        #[cfg(feature = "profiling")]
        profiling::memory::record_read(4);

        // Fast path if not crossing page boundary
        if (vaddr & 0xFFF) <= 0xFFC {
            let paddr = self.translate(vaddr, AccessType::Read, sregs)?;
            if let Some(p) = self.ram_ptr::<4>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+4) is in-RAM, non-MMIO.
                return Ok(u32::from_le(unsafe { (p as *const u32).read_unaligned() }));
            }
            let mut buf = [0u8; 4];
            self.read_phys(paddr, &mut buf)?;
            Ok(u32::from_le_bytes(buf))
        } else {
            let mut buf = [0u8; 4];
            self.read(vaddr, &mut buf, sregs)?;
            Ok(u32::from_le_bytes(buf))
        }
    }

    /// Read a u64 from virtual address.
    #[inline(always)]
    pub fn read_u64(&mut self, vaddr: u64, sregs: &SystemRegisters) -> Result<u64> {
        #[cfg(feature = "profiling")]
        profiling::memory::record_read(8);

        // Fast path if not crossing page boundary
        if (vaddr & 0xFFF) <= 0xFF8 {
            let paddr = self.translate(vaddr, AccessType::Read, sregs)?;
            if let Some(p) = self.ram_ptr::<8>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+8) is in-RAM, non-MMIO.
                return Ok(u64::from_le(unsafe { (p as *const u64).read_unaligned() }));
            }
            let mut buf = [0u8; 8];
            self.read_phys(paddr, &mut buf)?;
            Ok(u64::from_le_bytes(buf))
        } else {
            let mut buf = [0u8; 8];
            self.read(vaddr, &mut buf, sregs)?;
            Ok(u64::from_le_bytes(buf))
        }
    }

    /// Write a u8 to virtual address.
    #[inline(always)]
    pub fn write_u8(&mut self, vaddr: u64, value: u8, sregs: &SystemRegisters) -> Result<()> {
        self.note_smc(vaddr);
        #[cfg(feature = "profiling")]
        profiling::memory::record_write(1);

        let paddr = self.translate(vaddr, AccessType::Write, sregs)?;
        if let Some(p) = self.ram_ptr::<1>(paddr) {
            // SAFETY: ram_ptr verified [paddr, paddr+1) is in-RAM, non-MMIO.
            unsafe { (p as *mut u8).write(value) };
            return Ok(());
        }
        self.write_phys(paddr, &[value])
    }

    /// Write a u16 to virtual address.
    #[inline(always)]
    pub fn write_u16(&mut self, vaddr: u64, value: u16, sregs: &SystemRegisters) -> Result<()> {
        self.note_smc(vaddr);
        #[cfg(feature = "profiling")]
        profiling::memory::record_write(2);

        if (vaddr & 0xFFF) <= 0xFFE {
            let paddr = self.translate(vaddr, AccessType::Write, sregs)?;
            if let Some(p) = self.ram_ptr::<2>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+2) is in-RAM, non-MMIO.
                unsafe { (p as *mut u16).write_unaligned(value.to_le()) };
                return Ok(());
            }
            self.write_phys(paddr, &value.to_le_bytes())
        } else {
            self.write(vaddr, &value.to_le_bytes(), sregs)
        }
    }

    /// Write a u32 to virtual address.
    #[inline(always)]
    pub fn write_u32(&mut self, vaddr: u64, value: u32, sregs: &SystemRegisters) -> Result<()> {
        self.note_smc(vaddr);
        #[cfg(feature = "profiling")]
        profiling::memory::record_write(4);

        if (vaddr & 0xFFF) <= 0xFFC {
            let paddr = self.translate(vaddr, AccessType::Write, sregs)?;
            if let Some(p) = self.ram_ptr::<4>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+4) is in-RAM, non-MMIO.
                unsafe { (p as *mut u32).write_unaligned(value.to_le()) };
                return Ok(());
            }
            self.write_phys(paddr, &value.to_le_bytes())
        } else {
            self.write(vaddr, &value.to_le_bytes(), sregs)
        }
    }

    /// Write a u64 to virtual address.
    #[inline(always)]
    pub fn write_u64(&mut self, vaddr: u64, value: u64, sregs: &SystemRegisters) -> Result<()> {
        self.note_smc(vaddr);
        #[cfg(feature = "profiling")]
        profiling::memory::record_write(8);

        if (vaddr & 0xFFF) <= 0xFF8 {
            let paddr = self.translate(vaddr, AccessType::Write, sregs)?;
            if let Some(p) = self.ram_ptr::<8>(paddr) {
                // SAFETY: ram_ptr verified [paddr, paddr+8) is in-RAM, non-MMIO.
                unsafe { (p as *mut u64).write_unaligned(value.to_le()) };
                return Ok(());
            }
            self.write_phys(paddr, &value.to_le_bytes())
        } else {
            self.write(vaddr, &value.to_le_bytes(), sregs)
        }
    }
}
