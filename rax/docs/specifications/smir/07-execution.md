# SMIR Execution Model

This document specifies how SMIR is executed, including the interpreter, JIT interfaces, and runtime support.

## 1. Execution Context

```rust
/// Execution context for SMIR
pub struct SmirContext {
    /// Architecture being emulated
    pub source_arch: SourceArch,
    
    /// Virtual register file
    pub vregs: VRegFile,
    
    /// Architecture-specific register state
    pub arch_regs: ArchRegState,
    
    /// Flag state
    pub flags: FlagState,
    
    /// Memory interface
    pub memory: Box<dyn SmirMemory>,
    
    /// Current program counter (guest)
    pub pc: GuestAddr,
    
    /// Instruction count
    pub insn_count: u64,
    
    /// Cycle count (estimated)
    pub cycle_count: u64,
    
    /// Exit reason (when execution stops)
    pub exit_reason: Option<ExitReason>,
    
    /// Debug state
    pub debug: DebugState,
    
    /// Exclusive monitor (for ARM LL/SC)
    pub exclusive_monitor: ExclusiveMonitor,
    
    /// SMC tracker
    pub code_tracker: CodePageTracker,
}

/// Virtual register file
pub struct VRegFile {
    /// Virtual register values
    values: HashMap<VirtualId, u64>,
    
    /// Vector register values (128-bit)
    vec_values: HashMap<VirtualId, [u64; 2]>,
}

impl VRegFile {
    pub fn new() -> Self {
        VRegFile {
            values: HashMap::new(),
            vec_values: HashMap::new(),
        }
    }
    
    pub fn get(&self, vreg: VReg) -> u64 {
        match vreg {
            VReg::Virtual(id) => self.values.get(&id).copied().unwrap_or(0),
            VReg::Imm(val) => val as u64,
            VReg::Arch(_) => panic!("Use arch_regs for arch registers"),
        }
    }
    
    pub fn set(&mut self, vreg: VReg, value: u64) {
        match vreg {
            VReg::Virtual(id) => { self.values.insert(id, value); }
            VReg::Imm(_) => panic!("Cannot write to immediate"),
            VReg::Arch(_) => panic!("Use arch_regs for arch registers"),
        }
    }
    
    pub fn get_vec(&self, vreg: VReg) -> [u64; 2] {
        match vreg {
            VReg::Virtual(id) => self.vec_values.get(&id).copied().unwrap_or([0, 0]),
            _ => panic!("Vector access on non-virtual register"),
        }
    }
    
    pub fn set_vec(&mut self, vreg: VReg, value: [u64; 2]) {
        match vreg {
            VReg::Virtual(id) => { self.vec_values.insert(id, value); }
            _ => panic!("Vector write on non-virtual register"),
        }
    }
}

/// Architecture-specific register state
pub enum ArchRegState {
    X86_64(X86RegState),
    Aarch64(Aarch64RegState),
    Hexagon(HexagonRegState),
}

/// x86_64 register state
pub struct X86RegState {
    pub gpr: [u64; 16],      // RAX-R15
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
    pub fs_base: u64,
    pub gs_base: u64,
    pub xmm: [[u64; 2]; 32], // XMM0-XMM31
    pub ymm_hi: [[u64; 2]; 32], // Upper halves for YMM
    pub mxcsr: u32,
}

/// AArch64 register state
pub struct Aarch64RegState {
    pub x: [u64; 31],        // X0-X30
    pub sp: u64,
    pub pc: u64,
    pub nzcv: u32,
    pub fpcr: u32,
    pub fpsr: u32,
    pub v: [[u64; 2]; 32],   // V0-V31
}

/// Hexagon register state
pub struct HexagonRegState {
    pub r: [u32; 32],        // R0-R31
    pub p: [bool; 4],        // P0-P3
    pub pc: u32,
    pub gp: u32,
    pub lr: u32,
    pub sp: u32,
    pub lc: [u32; 2],        // LC0, LC1
    pub sa: [u32; 2],        // SA0, SA1
    pub usr: u32,
}

/// Exit reason
#[derive(Clone, Debug)]
pub enum ExitReason {
    /// Reached instruction limit
    InsnLimit,
    
    /// Halt instruction
    Halt,
    
    /// System call
    Syscall { num: u64, args: Vec<u64> },
    
    /// Breakpoint hit
    Breakpoint { addr: GuestAddr },
    
    /// Memory fault
    MemoryFault { error: MemoryError },
    
    /// Undefined instruction
    Undefined { addr: GuestAddr, bytes: Vec<u8> },
    
    /// External interrupt
    Interrupt { vector: u8 },
    
    /// Self-modifying code detected
    SmcDetected { addr: GuestAddr },
    
    /// Single step complete
    SingleStep,
    
    /// Function return (for call-return tracing)
    Return { to: GuestAddr },
}

/// Debug state
pub struct DebugState {
    /// Single-step mode
    pub single_step: bool,
    
    /// Breakpoints
    pub breakpoints: HashSet<GuestAddr>,
    
    /// Watchpoints (memory)
    pub watchpoints: Vec<Watchpoint>,
    
    /// Trace buffer
    pub trace: Option<TraceBuffer>,
}

/// Watchpoint definition
pub struct Watchpoint {
    pub addr: GuestAddr,
    pub size: usize,
    pub kind: WatchpointKind,
}

#[derive(Clone, Copy, Debug)]
pub enum WatchpointKind {
    Read,
    Write,
    ReadWrite,
}
```

## 2. Interpreter

```rust
/// SMIR interpreter
pub struct SmirInterpreter {
    /// Lifted code cache
    block_cache: HashMap<GuestAddr, SmirBlock>,
    
    /// Function cache
    func_cache: HashMap<GuestAddr, SmirFunction>,
    
    /// Lifter for on-demand translation
    lifter: Box<dyn SmirLifter>,
    
    /// Maximum instructions before yield
    max_insns_per_run: u64,
}

impl SmirInterpreter {
    pub fn new(arch: SourceArch) -> Self {
        SmirInterpreter {
            block_cache: HashMap::new(),
            func_cache: HashMap::new(),
            lifter: create_lifter(arch),
            max_insns_per_run: 10000,
        }
    }
    
    /// Run until exit condition
    pub fn run(&mut self, ctx: &mut SmirContext) -> ExitReason {
        let limit = ctx.insn_count + self.max_insns_per_run;
        
        loop {
            // Check instruction limit
            if ctx.insn_count >= limit {
                return ExitReason::InsnLimit;
            }
            
            // Check for pending exit
            if let Some(reason) = ctx.exit_reason.take() {
                return reason;
            }
            
            // Check breakpoints
            if ctx.debug.breakpoints.contains(&ctx.pc) {
                return ExitReason::Breakpoint { addr: ctx.pc };
            }
            
            // Get or lift block
            let block = match self.get_block(ctx.pc, ctx) {
                Ok(b) => b,
                Err(e) => {
                    return ExitReason::Undefined {
                        addr: ctx.pc,
                        bytes: vec![],
                    };
                }
            };
            
            // Execute block
            match self.execute_block(ctx, &block) {
                BlockResult::Continue(next_pc) => {
                    ctx.pc = next_pc;
                }
                BlockResult::Exit(reason) => {
                    return reason;
                }
            }
            
            // Single-step mode
            if ctx.debug.single_step {
                return ExitReason::SingleStep;
            }
        }
    }
    
    /// Get block from cache or lift
    fn get_block(&mut self, addr: GuestAddr, ctx: &SmirContext) -> Result<SmirBlock, LiftError> {
        if let Some(block) = self.block_cache.get(&addr) {
            return Ok(block.clone());
        }
        
        // Lift the block
        let mut lift_ctx = LiftContext {
            vreg_alloc: VRegAllocator::new(),
            block_alloc: BlockIdAllocator::new(),
            guest_pc: addr,
            endian: ctx.source_arch.default_endian(),
            known_functions: HashMap::new(),
            symbols: HashMap::new(),
            block_cache: HashMap::new(),
        };
        
        let block = self.lifter.lift_block(addr, ctx.memory.as_ref(), &mut lift_ctx)?;
        
        // Cache the block
        self.block_cache.insert(addr, block.clone());
        
        // Mark code page
        ctx.code_tracker.mark_code_page(addr);
        
        Ok(block)
    }
    
    /// Execute a single block
    fn execute_block(&self, ctx: &mut SmirContext, block: &SmirBlock) -> BlockResult {
        // Execute each operation
        for op in &block.ops {
            if let Err(e) = self.execute_op(ctx, op) {
                return BlockResult::Exit(ExitReason::MemoryFault { error: e });
            }
            ctx.insn_count += 1;
        }
        
        // Execute terminator
        self.execute_terminator(ctx, &block.terminator)
    }
    
    /// Execute a single operation
    fn execute_op(&self, ctx: &mut SmirContext, op: &SmirOp) -> Result<(), MemoryError> {
        match &op.kind {
            OpKind::Add { dst, src1, src2, width, flags } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = a.wrapping_add(b) & width.mask();
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Add,
                        result,
                        left: a,
                        right: b,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Sub { dst, src1, src2, width, flags } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = a.wrapping_sub(b) & width.mask();
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Sub,
                        result,
                        left: a,
                        right: b,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Cmp { src1, src2, width } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = a.wrapping_sub(b) & width.mask();
                
                ctx.flags.lazy = Some(LazyFlags {
                    op: LazyFlagOp::Sub,
                    result,
                    left: a,
                    right: b,
                    width: *width,
                    arch: ctx.source_arch,
                });
            }
            
            OpKind::And { dst, src1, src2, width, flags } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = (a & b) & width.mask();
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Logic,
                        result,
                        left: a,
                        right: b,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Or { dst, src1, src2, width, flags } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = (a | b) & width.mask();
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Logic,
                        result,
                        left: a,
                        right: b,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Xor { dst, src1, src2, width, flags } => {
                let a = self.read_vreg(ctx, *src1);
                let b = self.read_src_operand(ctx, src2);
                let result = (a ^ b) & width.mask();
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Logic,
                        result,
                        left: a,
                        right: b,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Shl { dst, src, amount, width, flags } => {
                let val = self.read_vreg(ctx, *src);
                let amt = self.read_src_operand(ctx, amount) & 0x3F;
                let result = if amt >= width.bits() as u64 {
                    0
                } else {
                    (val << amt) & width.mask()
                };
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Shl,
                        result,
                        left: val,
                        right: amt,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Shr { dst, src, amount, width, flags } => {
                let val = self.read_vreg(ctx, *src);
                let amt = self.read_src_operand(ctx, amount) & 0x3F;
                let result = if amt >= width.bits() as u64 {
                    0
                } else {
                    (val >> amt) & width.mask()
                };
                
                self.write_vreg(ctx, *dst, result);
                
                if *flags != FlagUpdate::None {
                    ctx.flags.lazy = Some(LazyFlags {
                        op: LazyFlagOp::Shr,
                        result,
                        left: val,
                        right: amt,
                        width: *width,
                        arch: ctx.source_arch,
                    });
                }
            }
            
            OpKind::Mov { dst, src, width } => {
                let val = self.read_src_operand(ctx, src) & width.mask();
                self.write_vreg(ctx, *dst, val);
            }
            
            OpKind::Load { dst, addr, width, sign } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = self.load_memory(ctx, effective_addr, *width, *sign)?;
                self.write_vreg(ctx, *dst, val);
            }
            
            OpKind::Store { src, addr, width } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = self.read_vreg(ctx, *src);
                self.store_memory(ctx, effective_addr, val, *width)?;
            }
            
            OpKind::Lea { dst, addr } => {
                let effective_addr = self.compute_address(ctx, addr);
                self.write_vreg(ctx, *dst, effective_addr);
            }
            
            OpKind::ZeroExtend { dst, src, from_width, to_width } => {
                let val = self.read_vreg(ctx, *src) & from_width.mask();
                self.write_vreg(ctx, *dst, val);
            }
            
            OpKind::SignExtend { dst, src, from_width, to_width } => {
                let val = self.read_vreg(ctx, *src) & from_width.mask();
                let sign_bit = 1u64 << (from_width.bits() - 1);
                let extended = if (val & sign_bit) != 0 {
                    val | !from_width.mask()
                } else {
                    val
                } & to_width.mask();
                self.write_vreg(ctx, *dst, extended);
            }
            
            OpKind::SetCC { dst, cond, width } => {
                let result = if ctx.flags.eval_condition(*cond) { 1 } else { 0 };
                self.write_vreg(ctx, *dst, result);
            }
            
            OpKind::CMove { dst, src, cond, width } => {
                if ctx.flags.eval_condition(*cond) {
                    let val = self.read_vreg(ctx, *src);
                    self.write_vreg(ctx, *dst, val);
                }
            }
            
            OpKind::Select { dst, cond, src_true, src_false, width } => {
                let cond_val = self.read_vreg(ctx, *cond);
                let result = if cond_val != 0 {
                    self.read_vreg(ctx, *src_true)
                } else {
                    self.read_vreg(ctx, *src_false)
                };
                self.write_vreg(ctx, *dst, result & width.mask());
            }
            
            OpKind::TestCondition { dst, cond } => {
                let result = if ctx.flags.eval_condition(*cond) { 1 } else { 0 };
                self.write_vreg(ctx, *dst, result);
            }
            
            OpKind::Nop => {}
            
            OpKind::Syscall { num, args } => {
                let num_val = self.read_vreg(ctx, *num);
                let arg_vals: Vec<u64> = args.iter()
                    .map(|a| self.read_vreg(ctx, *a))
                    .collect();
                ctx.exit_reason = Some(ExitReason::Syscall {
                    num: num_val,
                    args: arg_vals,
                });
            }
            
            OpKind::Fence { kind } => {
                ctx.memory.fence(*kind);
            }
            
            OpKind::AtomicLoad { dst, addr, width, order } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = ctx.memory.atomic_load(effective_addr, *width, *order)?;
                self.write_vreg(ctx, *dst, val);
            }
            
            OpKind::AtomicStore { src, addr, width, order } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = self.read_vreg(ctx, *src);
                ctx.memory.atomic_store(effective_addr, val, *width, *order)?;
            }
            
            OpKind::AtomicRmw { dst, addr, src, op, width, order } => {
                let effective_addr = self.compute_address(ctx, addr);
                let operand = self.read_vreg(ctx, *src);
                let old = ctx.memory.atomic_rmw(effective_addr, *op, operand, *width, *order)?;
                self.write_vreg(ctx, *dst, old);
            }
            
            OpKind::Cas { dst, success, addr, expected, new_val, width, order } => {
                let effective_addr = self.compute_address(ctx, addr);
                let exp = self.read_vreg(ctx, *expected);
                let new = self.read_vreg(ctx, *new_val);
                let (old, succ) = ctx.memory.compare_and_swap(
                    effective_addr, exp, new, *width, *order, MemoryOrder::Relaxed
                )?;
                self.write_vreg(ctx, *dst, old);
                self.write_vreg(ctx, *success, if succ { 1 } else { 0 });
            }
            
            OpKind::LoadExclusive { dst, addr, width } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = ctx.memory.load_exclusive(effective_addr, *width)?;
                ctx.exclusive_monitor.mark_exclusive(effective_addr, *width, val);
                self.write_vreg(ctx, *dst, val);
            }
            
            OpKind::StoreExclusive { status, src, addr, width } => {
                let effective_addr = self.compute_address(ctx, addr);
                let val = self.read_vreg(ctx, *src);
                let success = ctx.memory.store_exclusive(effective_addr, val, *width)?;
                self.write_vreg(ctx, *status, if success { 0 } else { 1 });
                ctx.exclusive_monitor.clear();
            }
            
            OpKind::ClearExclusive => {
                ctx.exclusive_monitor.clear();
                ctx.memory.clear_exclusive();
            }
            
            // FP operations
            OpKind::FAdd { dst, src1, src2, precision } => {
                let a = self.read_fp(ctx, *src1, *precision);
                let b = self.read_fp(ctx, *src2, *precision);
                let result = a + b;
                self.write_fp(ctx, *dst, result, *precision);
            }
            
            OpKind::FSub { dst, src1, src2, precision } => {
                let a = self.read_fp(ctx, *src1, *precision);
                let b = self.read_fp(ctx, *src2, *precision);
                let result = a - b;
                self.write_fp(ctx, *dst, result, *precision);
            }
            
            OpKind::FMul { dst, src1, src2, precision } => {
                let a = self.read_fp(ctx, *src1, *precision);
                let b = self.read_fp(ctx, *src2, *precision);
                let result = a * b;
                self.write_fp(ctx, *dst, result, *precision);
            }
            
            OpKind::FDiv { dst, src1, src2, precision } => {
                let a = self.read_fp(ctx, *src1, *precision);
                let b = self.read_fp(ctx, *src2, *precision);
                let result = a / b;
                self.write_fp(ctx, *dst, result, *precision);
            }
            
            // Architecture-specific ops
            OpKind::X86Specific(x86_op) => {
                self.execute_x86_specific(ctx, x86_op)?;
            }
            
            OpKind::ArmSpecific(arm_op) => {
                self.execute_arm_specific(ctx, arm_op)?;
            }
            
            OpKind::HexagonSpecific(hex_op) => {
                self.execute_hexagon_specific(ctx, hex_op)?;
            }
            
            _ => {
                // Unimplemented operation
                panic!("Unimplemented operation: {:?}", op.kind);
            }
        }
        
        Ok(())
    }
    
    /// Execute block terminator
    fn execute_terminator(&self, ctx: &mut SmirContext, term: &Terminator) -> BlockResult {
        match term {
            Terminator::Branch { target } => {
                // Look up target address from block ID
                // For now, assume block ID encodes address
                BlockResult::Continue(target.0 as u64) // Simplified
            }
            
            Terminator::CondBranch { cond, true_target, false_target } => {
                let cond_val = self.read_vreg(ctx, *cond);
                let target = if cond_val != 0 { true_target } else { false_target };
                BlockResult::Continue(target.0 as u64)
            }
            
            Terminator::IndirectBranch { target, .. } => {
                let addr = self.read_vreg(ctx, *target);
                BlockResult::Continue(addr)
            }
            
            Terminator::Return { .. } => {
                // Get return address from arch-specific location
                let ret_addr = match &ctx.arch_regs {
                    ArchRegState::X86_64(x86) => {
                        // Pop from stack
                        let rsp = x86.rsp;
                        let ret = ctx.memory.read(rsp, 8).ok()
                            .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
                            .unwrap_or(0);
                        // Update RSP
                        // (would need mutable access)
                        ret
                    }
                    ArchRegState::Aarch64(arm) => arm.x[30],
                    ArchRegState::Hexagon(hex) => hex.lr as u64,
                };
                BlockResult::Exit(ExitReason::Return { to: ret_addr })
            }
            
            Terminator::Trap { kind } => {
                match kind {
                    TrapKind::Halt => BlockResult::Exit(ExitReason::Halt),
                    TrapKind::Breakpoint => BlockResult::Exit(ExitReason::Breakpoint { addr: ctx.pc }),
                    TrapKind::SystemCall => {
                        // Already handled in Syscall op
                        BlockResult::Continue(ctx.pc)
                    }
                    _ => BlockResult::Exit(ExitReason::Undefined { addr: ctx.pc, bytes: vec![] }),
                }
            }
            
            Terminator::Unreachable => {
                BlockResult::Exit(ExitReason::Undefined { addr: ctx.pc, bytes: vec![] })
            }
            
            Terminator::Call { target, continuation, .. } => {
                // Handle call by jumping to target
                let target_addr = match target {
                    CallTarget::GuestAddr(addr) => *addr,
                    CallTarget::Direct(_) => unimplemented!(),
                    CallTarget::Indirect(reg) => self.read_vreg(ctx, *reg),
                    CallTarget::Runtime(_) => unimplemented!(),
                };
                BlockResult::Continue(target_addr)
            }
            
            _ => BlockResult::Continue(ctx.pc),
        }
    }
    
    /// Read a virtual register
    fn read_vreg(&self, ctx: &SmirContext, vreg: VReg) -> u64 {
        match vreg {
            VReg::Virtual(id) => ctx.vregs.values.get(&id).copied().unwrap_or(0),
            VReg::Imm(val) => val as u64,
            VReg::Arch(arch_reg) => self.read_arch_reg(ctx, arch_reg),
        }
    }
    
    /// Write a virtual register
    fn write_vreg(&self, ctx: &mut SmirContext, vreg: VReg, value: u64) {
        match vreg {
            VReg::Virtual(id) => { ctx.vregs.values.insert(id, value); }
            VReg::Imm(_) => panic!("Cannot write to immediate"),
            VReg::Arch(arch_reg) => self.write_arch_reg(ctx, arch_reg, value),
        }
    }
    
    /// Read architecture register
    fn read_arch_reg(&self, ctx: &SmirContext, reg: ArchReg) -> u64 {
        match (&ctx.arch_regs, reg) {
            (ArchRegState::X86_64(x86), ArchReg::X86(r)) => {
                match r {
                    X86Reg::Rax => x86.gpr[0],
                    X86Reg::Rcx => x86.gpr[1],
                    X86Reg::Rdx => x86.gpr[2],
                    X86Reg::Rbx => x86.gpr[3],
                    X86Reg::Rsp => x86.rsp,
                    X86Reg::Rbp => x86.gpr[5],
                    X86Reg::Rsi => x86.gpr[6],
                    X86Reg::Rdi => x86.gpr[7],
                    X86Reg::R8 => x86.gpr[8],
                    X86Reg::R9 => x86.gpr[9],
                    X86Reg::R10 => x86.gpr[10],
                    X86Reg::R11 => x86.gpr[11],
                    X86Reg::R12 => x86.gpr[12],
                    X86Reg::R13 => x86.gpr[13],
                    X86Reg::R14 => x86.gpr[14],
                    X86Reg::R15 => x86.gpr[15],
                    X86Reg::Rip => x86.rip,
                    X86Reg::Rflags => x86.rflags,
                    _ => 0,
                }
            }
            (ArchRegState::Aarch64(arm), ArchReg::Arm(r)) => {
                match r {
                    ArmReg::X(n) if n < 31 => arm.x[n as usize],
                    ArmReg::Sp => arm.sp,
                    ArmReg::Pc => arm.pc,
                    _ => 0,
                }
            }
            (ArchRegState::Hexagon(hex), ArchReg::Hexagon(r)) => {
                match r {
                    HexagonReg::R(n) => hex.r[n as usize] as u64,
                    HexagonReg::Pc => hex.pc as u64,
                    HexagonReg::Gp => hex.gp as u64,
                    HexagonReg::Lr => hex.lr as u64,
                    HexagonReg::Sp => hex.sp as u64,
                    _ => 0,
                }
            }
            _ => panic!("Architecture mismatch"),
        }
    }
    
    /// Write architecture register
    fn write_arch_reg(&self, ctx: &mut SmirContext, reg: ArchReg, value: u64) {
        match (&mut ctx.arch_regs, reg) {
            (ArchRegState::X86_64(x86), ArchReg::X86(r)) => {
                match r {
                    X86Reg::Rax => x86.gpr[0] = value,
                    X86Reg::Rcx => x86.gpr[1] = value,
                    X86Reg::Rdx => x86.gpr[2] = value,
                    X86Reg::Rbx => x86.gpr[3] = value,
                    X86Reg::Rsp => x86.rsp = value,
                    X86Reg::Rbp => x86.gpr[5] = value,
                    X86Reg::Rsi => x86.gpr[6] = value,
                    X86Reg::Rdi => x86.gpr[7] = value,
                    X86Reg::R8 => x86.gpr[8] = value,
                    X86Reg::R9 => x86.gpr[9] = value,
                    X86Reg::R10 => x86.gpr[10] = value,
                    X86Reg::R11 => x86.gpr[11] = value,
                    X86Reg::R12 => x86.gpr[12] = value,
                    X86Reg::R13 => x86.gpr[13] = value,
                    X86Reg::R14 => x86.gpr[14] = value,
                    X86Reg::R15 => x86.gpr[15] = value,
                    X86Reg::Rip => x86.rip = value,
                    _ => {}
                }
            }
            (ArchRegState::Aarch64(arm), ArchReg::Arm(r)) => {
                match r {
                    ArmReg::X(n) if n < 31 => arm.x[n as usize] = value,
                    ArmReg::Sp => arm.sp = value,
                    ArmReg::Pc => arm.pc = value,
                    _ => {}
                }
            }
            (ArchRegState::Hexagon(hex), ArchReg::Hexagon(r)) => {
                match r {
                    HexagonReg::R(n) => hex.r[n as usize] = value as u32,
                    HexagonReg::Pc => hex.pc = value as u32,
                    HexagonReg::Gp => hex.gp = value as u32,
                    HexagonReg::Lr => hex.lr = value as u32,
                    HexagonReg::Sp => hex.sp = value as u32,
                    _ => {}
                }
            }
            _ => panic!("Architecture mismatch"),
        }
    }
    
    /// Read source operand
    fn read_src_operand(&self, ctx: &SmirContext, src: &SrcOperand) -> u64 {
        match src {
            SrcOperand::Reg(r) => self.read_vreg(ctx, *r),
            SrcOperand::Imm(i) => *i as u64,
            SrcOperand::Shifted { reg, shift, amount } => {
                let val = self.read_vreg(ctx, *reg);
                match shift {
                    ShiftOp::Lsl => val << amount,
                    ShiftOp::Lsr => val >> amount,
                    ShiftOp::Asr => ((val as i64) >> amount) as u64,
                    ShiftOp::Ror => val.rotate_right(*amount as u32),
                    ShiftOp::Rrx => (val >> 1) | (if ctx.flags.get_cf() { 1 << 63 } else { 0 }),
                }
            }
            SrcOperand::Extended { reg, extend, shift } => {
                let val = self.read_vreg(ctx, *reg);
                let extended = match extend {
                    ExtendOp::Uxtb => val & 0xFF,
                    ExtendOp::Uxth => val & 0xFFFF,
                    ExtendOp::Uxtw => val & 0xFFFF_FFFF,
                    ExtendOp::Uxtx => val,
                    ExtendOp::Sxtb => ((val as i8) as i64) as u64,
                    ExtendOp::Sxth => ((val as i16) as i64) as u64,
                    ExtendOp::Sxtw => ((val as i32) as i64) as u64,
                    ExtendOp::Sxtx => val,
                };
                extended << shift
            }
        }
    }
    
    /// Compute effective address
    fn compute_address(&self, ctx: &SmirContext, addr: &Address) -> GuestAddr {
        compute_address(
            addr,
            &ContextRegAccess { ctx },
            ctx.pc,
            self.get_gp(ctx),
        )
    }
    
    /// Get GP register for Hexagon
    fn get_gp(&self, ctx: &SmirContext) -> Option<GuestAddr> {
        match &ctx.arch_regs {
            ArchRegState::Hexagon(hex) => Some(hex.gp as u64),
            _ => None,
        }
    }
    
    /// Load from memory
    fn load_memory(
        &self,
        ctx: &mut SmirContext,
        addr: GuestAddr,
        width: MemWidth,
        sign: SignExtend,
    ) -> Result<u64, MemoryError> {
        let mut buf = [0u8; 8];
        let size = width.bytes() as usize;
        ctx.memory.read(addr, &mut buf[..size])?;
        
        let raw = u64::from_le_bytes(buf);
        
        Ok(match sign {
            SignExtend::Zero => raw & ((1u64 << (size * 8)) - 1),
            SignExtend::Sign => {
                let shift = 64 - size * 8;
                ((raw as i64) << shift >> shift) as u64
            }
        })
    }
    
    /// Store to memory
    fn store_memory(
        &self,
        ctx: &mut SmirContext,
        addr: GuestAddr,
        value: u64,
        width: MemWidth,
    ) -> Result<(), MemoryError> {
        let bytes = value.to_le_bytes();
        let size = width.bytes() as usize;
        ctx.memory.write(addr, &bytes[..size])
    }
    
    /// Read FP register as f64
    fn read_fp(&self, ctx: &SmirContext, vreg: VReg, precision: FpPrecision) -> f64 {
        let bits = self.read_vreg(ctx, vreg);
        match precision {
            FpPrecision::F32 => f32::from_bits(bits as u32) as f64,
            FpPrecision::F64 => f64::from_bits(bits),
            _ => unimplemented!(),
        }
    }
    
    /// Write FP register from f64
    fn write_fp(&self, ctx: &mut SmirContext, vreg: VReg, value: f64, precision: FpPrecision) {
        let bits = match precision {
            FpPrecision::F32 => (value as f32).to_bits() as u64,
            FpPrecision::F64 => value.to_bits(),
            _ => unimplemented!(),
        };
        self.write_vreg(ctx, vreg, bits);
    }
    
    // Architecture-specific operation handlers
    fn execute_x86_specific(&self, ctx: &mut SmirContext, op: &X86SpecificOp) -> Result<(), MemoryError> {
        match op {
            X86SpecificOp::Rdtsc { edx_out, eax_out } => {
                let tsc = ctx.cycle_count;
                self.write_vreg(ctx, *eax_out, tsc & 0xFFFF_FFFF);
                self.write_vreg(ctx, *edx_out, tsc >> 32);
            }
            // ... other x86 ops
            _ => {}
        }
        Ok(())
    }
    
    fn execute_arm_specific(&self, ctx: &mut SmirContext, op: &ArmSpecificOp) -> Result<(), MemoryError> {
        match op {
            ArmSpecificOp::Svc { imm } => {
                ctx.exit_reason = Some(ExitReason::Syscall {
                    num: *imm as u64,
                    args: vec![],
                });
            }
            ArmSpecificOp::Wfi => {
                ctx.exit_reason = Some(ExitReason::Halt);
            }
            ArmSpecificOp::Isb => {
                ctx.memory.fence(FenceKind::ISync);
            }
            ArmSpecificOp::Dsb { .. } => {
                ctx.memory.fence(FenceKind::DSync);
            }
            ArmSpecificOp::Dmb { .. } => {
                ctx.memory.fence(FenceKind::Full);
            }
            _ => {}
        }
        Ok(())
    }
    
    fn execute_hexagon_specific(&self, ctx: &mut SmirContext, op: &HexagonSpecificOp) -> Result<(), MemoryError> {
        match op {
            HexagonSpecificOp::Trap { imm } => {
                ctx.exit_reason = Some(ExitReason::Syscall {
                    num: *imm as u64,
                    args: vec![],
                });
            }
            _ => {}
        }
        Ok(())
    }
}

/// Block execution result
enum BlockResult {
    Continue(GuestAddr),
    Exit(ExitReason),
}

/// Helper for register access during address computation
struct ContextRegAccess<'a> {
    ctx: &'a SmirContext,
}

impl<'a> RegisterAccess for ContextRegAccess<'a> {
    fn get(&self, vreg: VReg) -> u64 {
        match vreg {
            VReg::Virtual(id) => self.ctx.vregs.values.get(&id).copied().unwrap_or(0),
            VReg::Imm(val) => val as u64,
            VReg::Arch(arch_reg) => {
                // Simplified - would need full implementation
                0
            }
        }
    }
}

trait RegisterAccess {
    fn get(&self, vreg: VReg) -> u64;
}
```

## 3. Block ID Allocator

```rust
/// Block ID allocator
pub struct BlockIdAllocator {
    next_id: u32,
}

impl BlockIdAllocator {
    pub fn new() -> Self {
        BlockIdAllocator { next_id: 0 }
    }
    
    pub fn alloc(&mut self) -> BlockId {
        let id = self.next_id;
        self.next_id += 1;
        BlockId(id)
    }
}
```

## 4. JIT Interface (Future)

```rust
/// JIT compilation interface (placeholder)
pub trait SmirJit {
    /// Compile a function to native code
    fn compile_function(&mut self, func: &SmirFunction) -> Result<CompiledFunction, JitError>;
    
    /// Execute compiled code
    fn execute(&self, compiled: &CompiledFunction, ctx: &mut SmirContext) -> ExitReason;
    
    /// Invalidate compiled code for address range
    fn invalidate(&mut self, start: GuestAddr, end: GuestAddr);
}

/// Compiled function handle
pub struct CompiledFunction {
    /// Native code pointer
    code_ptr: *const u8,
    
    /// Code size
    code_size: usize,
    
    /// Guest address range
    guest_range: (GuestAddr, GuestAddr),
}

/// JIT compilation error
#[derive(Debug)]
pub enum JitError {
    Unsupported(String),
    Internal(String),
}
```

## 5. Performance Counters

```rust
/// Performance statistics
pub struct SmirStats {
    /// Instructions executed
    pub insn_count: u64,
    
    /// Blocks executed
    pub block_count: u64,
    
    /// Cache hits (block found in cache)
    pub cache_hits: u64,
    
    /// Cache misses (had to lift)
    pub cache_misses: u64,
    
    /// Memory loads
    pub load_count: u64,
    
    /// Memory stores
    pub store_count: u64,
    
    /// Branch count
    pub branch_count: u64,
    
    /// Syscall count
    pub syscall_count: u64,
}
```
