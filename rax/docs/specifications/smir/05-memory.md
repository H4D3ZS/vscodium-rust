# SMIR Memory Model

This document specifies how SMIR handles memory operations, addressing, atomics, and memory ordering.

## 1. Memory Abstraction

SMIR operates on a flat virtual address space. Address translation (page tables) is handled below the IR level.

```rust
/// Memory interface for SMIR execution
pub trait SmirMemory {
    /// Read bytes from memory
    fn read(&mut self, addr: GuestAddr, buf: &mut [u8]) -> Result<(), MemoryError>;
    
    /// Write bytes to memory
    fn write(&mut self, addr: GuestAddr, data: &[u8]) -> Result<(), MemoryError>;
    
    /// Atomic load
    fn atomic_load(&mut self, addr: GuestAddr, size: MemWidth, order: MemoryOrder) 
        -> Result<u64, MemoryError>;
    
    /// Atomic store
    fn atomic_store(&mut self, addr: GuestAddr, value: u64, size: MemWidth, order: MemoryOrder) 
        -> Result<(), MemoryError>;
    
    /// Atomic compare-and-swap
    fn compare_and_swap(
        &mut self, 
        addr: GuestAddr, 
        expected: u64, 
        new: u64, 
        size: MemWidth,
        success_order: MemoryOrder,
        failure_order: MemoryOrder,
    ) -> Result<(u64, bool), MemoryError>;
    
    /// Atomic read-modify-write
    fn atomic_rmw(
        &mut self,
        addr: GuestAddr,
        op: AtomicOp,
        operand: u64,
        size: MemWidth,
        order: MemoryOrder,
    ) -> Result<u64, MemoryError>;
    
    /// Load-linked / Load-exclusive
    fn load_exclusive(&mut self, addr: GuestAddr, size: MemWidth) -> Result<u64, MemoryError>;
    
    /// Store-conditional / Store-exclusive (returns true if succeeded)
    fn store_exclusive(&mut self, addr: GuestAddr, value: u64, size: MemWidth) 
        -> Result<bool, MemoryError>;
    
    /// Clear exclusive monitor
    fn clear_exclusive(&mut self);
    
    /// Memory fence
    fn fence(&mut self, kind: FenceKind);
    
    /// Prefetch hint
    fn prefetch(&mut self, addr: GuestAddr, locality: PrefetchLocality, write: bool);
    
    /// Check if address is valid and accessible
    fn probe(&self, addr: GuestAddr, size: usize, write: bool) -> Result<(), MemoryError>;
}

/// Memory access error
#[derive(Clone, Debug)]
pub enum MemoryError {
    /// Page fault (page not present)
    PageFault {
        addr: GuestAddr,
        write: bool,
        user: bool,
    },
    
    /// Access violation (permission denied)
    AccessViolation {
        addr: GuestAddr,
        required: AccessType,
    },
    
    /// Alignment fault
    Alignment {
        addr: GuestAddr,
        required: usize,
    },
    
    /// MMIO region (needs special handling)
    Mmio {
        addr: GuestAddr,
        size: usize,
    },
    
    /// Exclusive monitor failure
    ExclusiveFailed,
    
    /// Address out of bounds
    OutOfBounds {
        addr: GuestAddr,
    },
}

/// Access type for permission checks
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessType {
    Read,
    Write,
    Execute,
    ReadWrite,
}
```

## 2. Addressing Modes

```rust
/// Compute effective address
pub fn compute_address(
    addr: &Address,
    regs: &impl RegisterAccess,
    pc: GuestAddr,
    gp: Option<GuestAddr>,
) -> GuestAddr {
    match addr {
        Address::Direct(reg) => regs.get(*reg),
        
        Address::BaseOffset { base, offset } => {
            regs.get(*base).wrapping_add(*offset as u64)
        }
        
        Address::BaseIndexScale { base, index, scale, disp } => {
            let base_val = base.map(|r| regs.get(r)).unwrap_or(0);
            let index_val = regs.get(*index);
            base_val
                .wrapping_add(index_val.wrapping_mul(*scale as u64))
                .wrapping_add(*disp as i64 as u64)
        }
        
        Address::PcRel { offset } => {
            pc.wrapping_add(*offset as u64)
        }
        
        Address::GpRel { offset } => {
            gp.expect("GP register required for GP-relative addressing")
                .wrapping_add(*offset as i64 as u64)
        }
        
        Address::Absolute(addr) => *addr,
    }
}
```

## 3. Endianness

SMIR is **host-endian** internally. Endianness conversion happens at the lifting boundary.

```rust
/// Endianness for memory operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    Little,
    Big,
}

/// Convert bytes to u64 with specified endianness
pub fn bytes_to_u64(bytes: &[u8], endian: Endian) -> u64 {
    let mut arr = [0u8; 8];
    arr[..bytes.len()].copy_from_slice(bytes);
    match endian {
        Endian::Little => u64::from_le_bytes(arr),
        Endian::Big => u64::from_be_bytes(arr),
    }
}

/// Convert u64 to bytes with specified endianness
pub fn u64_to_bytes(value: u64, size: usize, endian: Endian) -> [u8; 8] {
    match endian {
        Endian::Little => value.to_le_bytes(),
        Endian::Big => value.to_be_bytes(),
    }
}

/// Architecture default endianness
impl SourceArch {
    pub fn default_endian(&self) -> Endian {
        match self {
            SourceArch::X86_64 => Endian::Little,
            SourceArch::Aarch64 => Endian::Little, // Default, can be big-endian
            SourceArch::Aarch32 => Endian::Little, // Default
            SourceArch::Hexagon => Endian::Little,
            SourceArch::RiscV64 | SourceArch::RiscV32 => Endian::Little,
            _ => Endian::Little,
        }
    }
}
```

## 4. Alignment Requirements

```rust
/// Check alignment for memory access
pub fn check_alignment(addr: GuestAddr, size: MemWidth, strict: bool) -> Result<(), MemoryError> {
    let required = if strict {
        size.bytes() as u64
    } else {
        // Natural alignment: up to 8 bytes
        (size.bytes() as u64).min(8)
    };
    
    if addr % required != 0 {
        Err(MemoryError::Alignment {
            addr,
            required: required as usize,
        })
    } else {
        Ok(())
    }
}

/// Architecture alignment strictness
impl SourceArch {
    pub fn alignment_strict(&self) -> bool {
        match self {
            SourceArch::X86_64 => false,  // x86 allows unaligned (slower)
            SourceArch::Aarch64 => true,  // ARM requires alignment (unless SCTLR.A=0)
            SourceArch::Aarch32 => true,
            SourceArch::Hexagon => true,
            _ => true,
        }
    }
}
```

## 5. Memory Ordering

SMIR uses a memory ordering model that can represent both x86 TSO and ARM weak ordering.

```rust
/// Memory ordering constraints
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryOrder {
    /// No ordering constraints
    Relaxed,
    
    /// Acquire: subsequent reads/writes cannot be reordered before this load
    Acquire,
    
    /// Release: previous reads/writes cannot be reordered after this store
    Release,
    
    /// Acquire-release: both acquire and release semantics
    AcqRel,
    
    /// Sequentially consistent: total ordering
    SeqCst,
}

impl MemoryOrder {
    /// Check if this order is at least as strong as another
    pub fn is_at_least(&self, other: MemoryOrder) -> bool {
        *self >= other
    }
    
    /// Combine two orderings (take the stronger)
    pub fn combine(&self, other: MemoryOrder) -> MemoryOrder {
        (*self).max(other)
    }
}

/// Fence operations
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FenceKind {
    /// Load-load barrier (rare, mostly for weakly-ordered archs)
    LoadLoad,
    
    /// Load-store barrier
    LoadStore,
    
    /// Store-load barrier (most expensive, required for TSO violations)
    StoreLoad,
    
    /// Store-store barrier
    StoreStore,
    
    /// Full barrier (all of the above)
    Full,
    
    /// Instruction synchronization (x86 LFENCE, ARM ISB)
    ISync,
    
    /// Data synchronization (ARM DSB)
    DSync,
}

impl FenceKind {
    /// Get barriers implied by this fence
    pub fn barriers(&self) -> (bool, bool, bool, bool) {
        // (load-load, load-store, store-load, store-store)
        match self {
            FenceKind::LoadLoad => (true, false, false, false),
            FenceKind::LoadStore => (false, true, false, false),
            FenceKind::StoreLoad => (false, false, true, false),
            FenceKind::StoreStore => (false, false, false, true),
            FenceKind::Full => (true, true, true, true),
            FenceKind::ISync | FenceKind::DSync => (true, true, true, true),
        }
    }
}
```

## 6. Atomic Operations

```rust
/// Execute atomic load
pub fn exec_atomic_load(
    mem: &mut impl SmirMemory,
    dst: &mut u64,
    addr: GuestAddr,
    width: MemWidth,
    order: MemoryOrder,
) -> Result<(), MemoryError> {
    // Check alignment (atomics always require natural alignment)
    check_alignment(addr, width, true)?;
    
    *dst = mem.atomic_load(addr, width, order)?;
    Ok(())
}

/// Execute atomic store
pub fn exec_atomic_store(
    mem: &mut impl SmirMemory,
    addr: GuestAddr,
    src: u64,
    width: MemWidth,
    order: MemoryOrder,
) -> Result<(), MemoryError> {
    check_alignment(addr, width, true)?;
    mem.atomic_store(addr, src, width, order)
}

/// Execute atomic read-modify-write
pub fn exec_atomic_rmw(
    mem: &mut impl SmirMemory,
    dst: &mut u64,
    addr: GuestAddr,
    src: u64,
    op: AtomicOp,
    width: MemWidth,
    order: MemoryOrder,
) -> Result<(), MemoryError> {
    check_alignment(addr, width, true)?;
    
    *dst = mem.atomic_rmw(addr, op, src, width, order)?;
    Ok(())
}

/// Execute compare-and-swap
pub fn exec_cas(
    mem: &mut impl SmirMemory,
    old_val: &mut u64,
    success: &mut bool,
    addr: GuestAddr,
    expected: u64,
    new_val: u64,
    width: MemWidth,
    order: MemoryOrder,
) -> Result<(), MemoryError> {
    check_alignment(addr, width, true)?;
    
    let (old, succ) = mem.compare_and_swap(
        addr, expected, new_val, width, order, MemoryOrder::Relaxed
    )?;
    *old_val = old;
    *success = succ;
    Ok(())
}
```

## 7. Exclusive Monitor (ARM LL/SC)

```rust
/// Exclusive monitor state
pub struct ExclusiveMonitor {
    /// Address being monitored (None if no exclusive)
    addr: Option<GuestAddr>,
    
    /// Size of exclusive region
    size: MemWidth,
    
    /// Value loaded (for comparison on store)
    value: u64,
}

impl ExclusiveMonitor {
    pub fn new() -> Self {
        ExclusiveMonitor {
            addr: None,
            size: MemWidth::B8,
            value: 0,
        }
    }
    
    /// Mark exclusive access (LDXR)
    pub fn mark_exclusive(&mut self, addr: GuestAddr, size: MemWidth, value: u64) {
        self.addr = Some(addr);
        self.size = size;
        self.value = value;
    }
    
    /// Check if exclusive is still valid for this address
    pub fn check_exclusive(&self, addr: GuestAddr, size: MemWidth) -> bool {
        match self.addr {
            Some(excl_addr) => {
                excl_addr == addr && self.size == size
            }
            None => false,
        }
    }
    
    /// Clear exclusive monitor
    pub fn clear(&mut self) {
        self.addr = None;
    }
    
    /// Check if a store to this address should clear the monitor
    pub fn should_clear_for_store(&self, addr: GuestAddr, size: usize) -> bool {
        if let Some(excl_addr) = self.addr {
            // Clear if ranges overlap
            let excl_end = excl_addr + self.size.bytes() as u64;
            let store_end = addr + size as u64;
            !(store_end <= excl_addr || addr >= excl_end)
        } else {
            false
        }
    }
}
```

## 8. Memory-Mapped I/O

```rust
/// MMIO region descriptor
pub struct MmioRegion {
    pub base: GuestAddr,
    pub size: u64,
    pub handler: Box<dyn MmioHandler>,
}

/// MMIO handler interface
pub trait MmioHandler: Send {
    fn read(&mut self, offset: u64, size: MemWidth) -> Result<u64, MemoryError>;
    fn write(&mut self, offset: u64, value: u64, size: MemWidth) -> Result<(), MemoryError>;
}

/// Check if address is in MMIO region
pub fn is_mmio(addr: GuestAddr, regions: &[MmioRegion]) -> Option<&MmioRegion> {
    regions.iter().find(|r| {
        addr >= r.base && addr < r.base + r.size
    })
}
```

## 9. Self-Modifying Code Detection

```rust
/// Code page tracking for SMC detection
pub struct CodePageTracker {
    /// Set of pages containing lifted code
    code_pages: HashSet<u64>,
    
    /// Page size (4KB typically)
    page_size: u64,
}

impl CodePageTracker {
    pub fn new(page_size: u64) -> Self {
        CodePageTracker {
            code_pages: HashSet::new(),
            page_size,
        }
    }
    
    /// Mark a page as containing code
    pub fn mark_code_page(&mut self, addr: GuestAddr) {
        let page = addr / self.page_size;
        self.code_pages.insert(page);
    }
    
    /// Check if a store hits a code page (SMC)
    pub fn check_write(&self, addr: GuestAddr, size: usize) -> bool {
        let start_page = addr / self.page_size;
        let end_page = (addr + size as u64 - 1) / self.page_size;
        
        for page in start_page..=end_page {
            if self.code_pages.contains(&page) {
                return true;
            }
        }
        false
    }
    
    /// Invalidate a page (e.g., after SMC detected)
    pub fn invalidate_page(&mut self, addr: GuestAddr) {
        let page = addr / self.page_size;
        self.code_pages.remove(&page);
    }
}
```

## 10. Memory Operation Execution

```rust
/// Execute a load operation
pub fn exec_load(
    mem: &mut impl SmirMemory,
    dst: &mut u64,
    addr: GuestAddr,
    width: MemWidth,
    sign: SignExtend,
    endian: Endian,
    strict_align: bool,
) -> Result<(), MemoryError> {
    // Check alignment
    if strict_align {
        check_alignment(addr, width, true)?;
    }
    
    // Read bytes
    let size = width.bytes() as usize;
    let mut buf = [0u8; 8];
    mem.read(addr, &mut buf[..size])?;
    
    // Convert to value
    let raw = bytes_to_u64(&buf[..size], endian);
    
    // Sign or zero extend
    *dst = match sign {
        SignExtend::Zero => raw,
        SignExtend::Sign => {
            let shift = 64 - width.bytes() as u32 * 8;
            ((raw as i64) << shift >> shift) as u64
        }
    };
    
    Ok(())
}

/// Execute a store operation
pub fn exec_store(
    mem: &mut impl SmirMemory,
    addr: GuestAddr,
    src: u64,
    width: MemWidth,
    endian: Endian,
    strict_align: bool,
    code_tracker: &CodePageTracker,
) -> Result<bool, MemoryError> {
    // Check alignment
    if strict_align {
        check_alignment(addr, width, true)?;
    }
    
    // Check for SMC
    let smc = code_tracker.check_write(addr, width.bytes() as usize);
    
    // Convert to bytes
    let bytes = u64_to_bytes(src, width.bytes() as usize, endian);
    
    // Write
    mem.write(addr, &bytes[..width.bytes() as usize])?;
    
    // Return whether SMC was detected
    Ok(smc)
}
```

## 11. Stack Operations

```rust
/// Push value to stack (x86-style: decrement then store)
pub fn push_x86(
    mem: &mut impl SmirMemory,
    sp: &mut u64,
    value: u64,
    width: MemWidth,
) -> Result<(), MemoryError> {
    let size = width.bytes() as u64;
    *sp = sp.wrapping_sub(size);
    exec_store(mem, *sp, value, width, Endian::Little, false, &CodePageTracker::new(4096))
        .map(|_| ())
}

/// Pop value from stack (x86-style: load then increment)
pub fn pop_x86(
    mem: &mut impl SmirMemory,
    sp: &mut u64,
    dst: &mut u64,
    width: MemWidth,
) -> Result<(), MemoryError> {
    exec_load(mem, dst, *sp, width, SignExtend::Zero, Endian::Little, false)?;
    let size = width.bytes() as u64;
    *sp = sp.wrapping_add(size);
    Ok(())
}

/// ARM-style push (store then decrement) - pre-decrement
pub fn push_arm(
    mem: &mut impl SmirMemory,
    sp: &mut u64,
    value: u64,
    width: MemWidth,
) -> Result<(), MemoryError> {
    let size = width.bytes() as u64;
    *sp = sp.wrapping_sub(size);
    exec_store(mem, *sp, value, width, Endian::Little, true, &CodePageTracker::new(4096))
        .map(|_| ())
}
```

## 12. String Operations (x86 REP)

```rust
/// x86 string operation state
pub struct StringOpState {
    /// Source address (RSI for x86)
    pub src_addr: GuestAddr,
    
    /// Destination address (RDI for x86)
    pub dst_addr: GuestAddr,
    
    /// Count register (RCX for x86)
    pub count: u64,
    
    /// Direction (-1 or +1)
    pub direction: i64,
    
    /// Element size
    pub elem_size: u64,
}

/// Execute REP MOVSB/MOVSW/MOVSD/MOVSQ
pub fn exec_rep_movs(
    mem: &mut impl SmirMemory,
    state: &mut StringOpState,
    max_iterations: u64,
) -> Result<u64, MemoryError> {
    let mut iterations = 0u64;
    
    while state.count > 0 && iterations < max_iterations {
        // Read from source
        let mut buf = [0u8; 8];
        mem.read(state.src_addr, &mut buf[..state.elem_size as usize])?;
        
        // Write to destination
        mem.write(state.dst_addr, &buf[..state.elem_size as usize])?;
        
        // Update pointers
        let delta = state.direction * state.elem_size as i64;
        state.src_addr = state.src_addr.wrapping_add(delta as u64);
        state.dst_addr = state.dst_addr.wrapping_add(delta as u64);
        state.count -= 1;
        iterations += 1;
    }
    
    Ok(iterations)
}

/// Execute REP STOSB/STOSW/STOSD/STOSQ
pub fn exec_rep_stos(
    mem: &mut impl SmirMemory,
    state: &mut StringOpState,
    value: u64,
    max_iterations: u64,
) -> Result<u64, MemoryError> {
    let mut iterations = 0u64;
    let bytes = value.to_le_bytes();
    
    while state.count > 0 && iterations < max_iterations {
        mem.write(state.dst_addr, &bytes[..state.elem_size as usize])?;
        
        let delta = state.direction * state.elem_size as i64;
        state.dst_addr = state.dst_addr.wrapping_add(delta as u64);
        state.count -= 1;
        iterations += 1;
    }
    
    Ok(iterations)
}
```
