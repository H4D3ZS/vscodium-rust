# SMIR Lifting Interfaces

This document specifies the interfaces and algorithms for lifting machine code to SMIR.

## 1. Lifter Trait

```rust
/// Lifter interface for converting machine code to SMIR
pub trait SmirLifter {
    /// Source architecture
    fn source_arch(&self) -> SourceArch;
    
    /// Lift a single instruction at the given address
    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError>;
    
    /// Lift a basic block starting at the given address
    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError>;
    
    /// Lift a function (all reachable blocks from entry)
    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError>;
}

/// Result of lifting a single instruction
pub struct LiftResult {
    /// SMIR operations generated
    pub ops: Vec<SmirOp>,
    
    /// Number of bytes consumed
    pub bytes_consumed: usize,
    
    /// Control flow effect
    pub control_flow: ControlFlow,
    
    /// Branch targets (for block discovery)
    pub branch_targets: Vec<GuestAddr>,
}

/// Control flow after an instruction
#[derive(Clone, Debug)]
pub enum ControlFlow {
    /// Continue to next instruction
    Fallthrough,
    
    /// Unconditional branch
    Branch { target: GuestAddr },
    
    /// Conditional branch
    CondBranch { 
        cond: Condition, 
        target: GuestAddr, 
        fallthrough: GuestAddr,
    },
    
    /// Indirect branch (computed target)
    IndirectBranch,
    
    /// Function call
    Call { target: CallTarget },
    
    /// Return from function
    Return,
    
    /// Trap (exception, undefined)
    Trap { kind: TrapKind },
    
    /// System call
    Syscall,
}

/// Lifting context (shared state across lifting)
pub struct LiftContext {
    /// Virtual register allocator
    pub vreg_alloc: VRegAllocator,
    
    /// Block ID allocator
    pub block_alloc: BlockIdAllocator,
    
    /// Current guest PC
    pub guest_pc: GuestAddr,
    
    /// Endianness
    pub endian: Endian,
    
    /// Known function entries (for call resolution)
    pub known_functions: HashMap<GuestAddr, FunctionId>,
    
    /// Symbol table
    pub symbols: HashMap<GuestAddr, String>,
    
    /// Lifted blocks cache
    pub block_cache: HashMap<GuestAddr, BlockId>,
}

/// Lifting error
#[derive(Clone, Debug)]
pub enum LiftError {
    /// Invalid instruction encoding
    InvalidEncoding { addr: GuestAddr, bytes: Vec<u8> },
    
    /// Unsupported instruction
    Unsupported { addr: GuestAddr, mnemonic: String },
    
    /// Memory read error
    MemoryError { addr: GuestAddr, error: MemoryError },
    
    /// Incomplete instruction (need more bytes)
    Incomplete { addr: GuestAddr, have: usize, need: usize },
    
    /// Internal lifter error
    Internal(String),
}

/// Memory reader interface for lifting
pub trait MemoryReader {
    fn read(&self, addr: GuestAddr, size: usize) -> Result<Vec<u8>, MemoryError>;
}
```

## 2. Virtual Register Allocation

```rust
/// Virtual register allocator
pub struct VRegAllocator {
    next_id: u32,
    /// Mapping from arch registers to current virtual registers
    arch_to_vreg: HashMap<ArchReg, VirtualId>,
}

impl VRegAllocator {
    pub fn new() -> Self {
        VRegAllocator {
            next_id: 0,
            arch_to_vreg: HashMap::new(),
        }
    }
    
    /// Allocate a new virtual register
    pub fn alloc(&mut self) -> VReg {
        let id = self.next_id;
        self.next_id += 1;
        VReg::Virtual(VirtualId(id))
    }
    
    /// Get the current virtual register for an arch register
    pub fn get_arch(&self, reg: ArchReg) -> VReg {
        self.arch_to_vreg
            .get(&reg)
            .map(|id| VReg::Virtual(*id))
            .unwrap_or(VReg::Arch(reg))
    }
    
    /// Define a new value for an arch register
    pub fn define_arch(&mut self, reg: ArchReg) -> VReg {
        let vreg = self.alloc();
        if let VReg::Virtual(id) = vreg {
            self.arch_to_vreg.insert(reg, id);
        }
        vreg
    }
    
    /// Reset mappings (for new block entry)
    pub fn reset(&mut self) {
        self.arch_to_vreg.clear();
    }
}
```

## 3. x86_64 Lifter

```rust
/// x86_64 instruction lifter
pub struct X86Lifter {
    /// Decoder state
    decoder: X86Decoder,
    
    /// Cached decode results
    decode_cache: HashMap<GuestAddr, X86DecodedInsn>,
}

impl SmirLifter for X86Lifter {
    fn source_arch(&self) -> SourceArch {
        SourceArch::X86_64
    }
    
    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        // Decode the instruction
        let decoded = self.decoder.decode(bytes)?;
        
        ctx.guest_pc = addr;
        let mut ops = Vec::new();
        let mut branch_targets = Vec::new();
        
        // Map based on opcode
        let control_flow = match &decoded {
            X86DecodedInsn::Add { dst, src, width } => {
                self.lift_add(&mut ops, ctx, dst, src, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Sub { dst, src, width } => {
                self.lift_sub(&mut ops, ctx, dst, src, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Mov { dst, src, width } => {
                self.lift_mov(&mut ops, ctx, dst, src, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Cmp { src1, src2, width } => {
                self.lift_cmp(&mut ops, ctx, src1, src2, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Jcc { cond, target } => {
                let smir_cond = self.x86_cond_to_smir(*cond);
                branch_targets.push(*target);
                branch_targets.push(addr + decoded.len() as u64);
                ControlFlow::CondBranch {
                    cond: smir_cond,
                    target: *target,
                    fallthrough: addr + decoded.len() as u64,
                }
            }
            
            X86DecodedInsn::Jmp { target } => {
                match target {
                    X86Target::Direct(addr) => {
                        branch_targets.push(*addr);
                        ControlFlow::Branch { target: *addr }
                    }
                    X86Target::Indirect(operand) => {
                        self.lift_indirect_jmp(&mut ops, ctx, operand)?;
                        ControlFlow::IndirectBranch
                    }
                }
            }
            
            X86DecodedInsn::Call { target } => {
                // Push return address
                let ret_addr = addr + decoded.len() as u64;
                self.lift_push_imm(&mut ops, ctx, ret_addr)?;
                
                match target {
                    X86Target::Direct(target_addr) => {
                        branch_targets.push(*target_addr);
                        ControlFlow::Call { 
                            target: CallTarget::GuestAddr(*target_addr)
                        }
                    }
                    X86Target::Indirect(operand) => {
                        let target_vreg = self.lift_operand_read(&mut ops, ctx, operand, OpWidth::W64)?;
                        ControlFlow::Call {
                            target: CallTarget::Indirect(target_vreg)
                        }
                    }
                }
            }
            
            X86DecodedInsn::Ret { pop_bytes } => {
                self.lift_ret(&mut ops, ctx, *pop_bytes)?;
                ControlFlow::Return
            }
            
            X86DecodedInsn::Push { src, width } => {
                self.lift_push(&mut ops, ctx, src, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Pop { dst, width } => {
                self.lift_pop(&mut ops, ctx, dst, *width)?;
                ControlFlow::Fallthrough
            }
            
            X86DecodedInsn::Lea { dst, src } => {
                self.lift_lea(&mut ops, ctx, dst, src)?;
                ControlFlow::Fallthrough
            }
            
            // ... many more opcodes
            
            X86DecodedInsn::Syscall => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Syscall {
                        num: VReg::Arch(ArchReg::X86(X86Reg::Rax)),
                        args: vec![
                            VReg::Arch(ArchReg::X86(X86Reg::Rdi)),
                            VReg::Arch(ArchReg::X86(X86Reg::Rsi)),
                            VReg::Arch(ArchReg::X86(X86Reg::Rdx)),
                            VReg::Arch(ArchReg::X86(X86Reg::R10)),
                            VReg::Arch(ArchReg::X86(X86Reg::R8)),
                            VReg::Arch(ArchReg::X86(X86Reg::R9)),
                        ],
                    },
                });
                ControlFlow::Syscall
            }
            
            X86DecodedInsn::Nop => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Nop,
                });
                ControlFlow::Fallthrough
            }
            
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: decoded.mnemonic().to_string(),
                });
            }
        };
        
        Ok(LiftResult {
            ops,
            bytes_consumed: decoded.len(),
            control_flow,
            branch_targets,
        })
    }
    
    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError> {
        let block_id = ctx.block_alloc.alloc();
        let mut ops = Vec::new();
        let mut current_addr = addr;
        
        loop {
            // Fetch instruction bytes
            let bytes = mem.read(current_addr, 15)
                .map_err(|e| LiftError::MemoryError { addr: current_addr, error: e })?;
            
            // Lift the instruction
            let result = self.lift_insn(current_addr, &bytes, ctx)?;
            
            ops.extend(result.ops);
            current_addr += result.bytes_consumed as u64;
            
            // Check if block ends
            match result.control_flow {
                ControlFlow::Fallthrough => continue,
                
                ControlFlow::Branch { target } => {
                    let target_block = ctx.block_cache.get(&target)
                        .copied()
                        .unwrap_or_else(|| ctx.block_alloc.alloc());
                    
                    return Ok(SmirBlock {
                        id: block_id,
                        guest_pc: addr,
                        phis: vec![],
                        ops,
                        terminator: Terminator::Branch { target: target_block },
                        exec_count: 0,
                    });
                }
                
                ControlFlow::CondBranch { cond, target, fallthrough } => {
                    let true_block = ctx.block_cache.get(&target)
                        .copied()
                        .unwrap_or_else(|| ctx.block_alloc.alloc());
                    let false_block = ctx.block_cache.get(&fallthrough)
                        .copied()
                        .unwrap_or_else(|| ctx.block_alloc.alloc());
                    
                    // Need to materialize condition
                    let cond_vreg = ctx.vreg_alloc.alloc();
                    ops.push(SmirOp {
                        id: OpId(ops.len() as u16),
                        guest_pc: current_addr,
                        kind: OpKind::TestCondition { dst: cond_vreg, cond },
                    });
                    
                    return Ok(SmirBlock {
                        id: block_id,
                        guest_pc: addr,
                        phis: vec![],
                        ops,
                        terminator: Terminator::CondBranch {
                            cond: cond_vreg,
                            true_target: true_block,
                            false_target: false_block,
                        },
                        exec_count: 0,
                    });
                }
                
                ControlFlow::Return => {
                    return Ok(SmirBlock {
                        id: block_id,
                        guest_pc: addr,
                        phis: vec![],
                        ops,
                        terminator: Terminator::Return { values: vec![] },
                        exec_count: 0,
                    });
                }
                
                ControlFlow::Trap { kind } => {
                    return Ok(SmirBlock {
                        id: block_id,
                        guest_pc: addr,
                        phis: vec![],
                        ops,
                        terminator: Terminator::Trap { kind },
                        exec_count: 0,
                    });
                }
                
                _ => {
                    // Handle other control flow types...
                    break;
                }
            }
        }
        
        Err(LiftError::Internal("Unexpected end of block".to_string()))
    }
    
    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError> {
        let func_id = FunctionId(ctx.known_functions.len() as u32);
        let mut blocks = Vec::new();
        let mut worklist = vec![entry];
        let mut visited = HashSet::new();
        
        while let Some(addr) = worklist.pop() {
            if visited.contains(&addr) {
                continue;
            }
            visited.insert(addr);
            
            let block = self.lift_block(addr, mem, ctx)?;
            
            // Add branch targets to worklist
            match &block.terminator {
                Terminator::Branch { target } => {
                    // TODO: resolve target address from block ID
                }
                Terminator::CondBranch { true_target, false_target, .. } => {
                    // TODO: add targets to worklist
                }
                _ => {}
            }
            
            blocks.push(block);
        }
        
        Ok(SmirFunction {
            id: func_id,
            entry: blocks.first().map(|b| b.id).unwrap_or(BlockId(0)),
            blocks,
            locals: vec![],
            guest_range: (entry, entry), // TODO: compute actual range
            calling_convention: CallingConv::X86SysV,
            attrs: FunctionAttrs::default(),
        })
    }
}

impl X86Lifter {
    /// Lift ADD instruction
    fn lift_add(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        dst: &X86Operand,
        src: &X86Operand,
        width: OpWidth,
    ) -> Result<(), LiftError> {
        let src1 = self.lift_operand_read(ops, ctx, dst, width)?;
        let src2 = self.lift_operand_to_srcop(ops, ctx, src, width)?;
        let result = ctx.vreg_alloc.alloc();
        
        ops.push(SmirOp {
            id: OpId(ops.len() as u16),
            guest_pc: ctx.guest_pc,
            kind: OpKind::Add {
                dst: result,
                src1,
                src2,
                width,
                flags: FlagUpdate::All,
            },
        });
        
        self.lift_operand_write(ops, ctx, dst, result, width)?;
        Ok(())
    }
    
    /// Lift CMP instruction
    fn lift_cmp(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        src1: &X86Operand,
        src2: &X86Operand,
        width: OpWidth,
    ) -> Result<(), LiftError> {
        let left = self.lift_operand_read(ops, ctx, src1, width)?;
        let right = self.lift_operand_to_srcop(ops, ctx, src2, width)?;
        
        ops.push(SmirOp {
            id: OpId(ops.len() as u16),
            guest_pc: ctx.guest_pc,
            kind: OpKind::Cmp {
                src1: left,
                src2: right,
                width,
            },
        });
        
        Ok(())
    }
    
    /// Read an operand into a virtual register
    fn lift_operand_read(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        operand: &X86Operand,
        width: OpWidth,
    ) -> Result<VReg, LiftError> {
        match operand {
            X86Operand::Reg(reg) => {
                Ok(VReg::Arch(ArchReg::X86(*reg)))
            }
            
            X86Operand::Mem(mem) => {
                let addr = self.lift_memory_address(ops, ctx, mem)?;
                let dst = ctx.vreg_alloc.alloc();
                
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: ctx.guest_pc,
                    kind: OpKind::Load {
                        dst,
                        addr,
                        width: width.to_mem_width(),
                        sign: SignExtend::Zero,
                    },
                });
                
                Ok(dst)
            }
            
            X86Operand::Imm(imm) => {
                Ok(VReg::Imm(*imm))
            }
        }
    }
    
    /// Write a virtual register to an operand
    fn lift_operand_write(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        operand: &X86Operand,
        src: VReg,
        width: OpWidth,
    ) -> Result<(), LiftError> {
        match operand {
            X86Operand::Reg(reg) => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: ctx.guest_pc,
                    kind: OpKind::Mov {
                        dst: VReg::Arch(ArchReg::X86(*reg)),
                        src: SrcOperand::Reg(src),
                        width,
                    },
                });
            }
            
            X86Operand::Mem(mem) => {
                let addr = self.lift_memory_address(ops, ctx, mem)?;
                
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: ctx.guest_pc,
                    kind: OpKind::Store {
                        src,
                        addr,
                        width: width.to_mem_width(),
                    },
                });
            }
            
            X86Operand::Imm(_) => {
                return Err(LiftError::Internal("Cannot write to immediate".to_string()));
            }
        }
        
        Ok(())
    }
    
    /// Convert operand to SrcOperand
    fn lift_operand_to_srcop(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        operand: &X86Operand,
        width: OpWidth,
    ) -> Result<SrcOperand, LiftError> {
        match operand {
            X86Operand::Imm(imm) => Ok(SrcOperand::Imm(*imm)),
            _ => {
                let vreg = self.lift_operand_read(ops, ctx, operand, width)?;
                Ok(SrcOperand::Reg(vreg))
            }
        }
    }
    
    /// Lift memory address operand to SMIR Address
    fn lift_memory_address(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        mem: &X86MemOperand,
    ) -> Result<Address, LiftError> {
        match mem {
            X86MemOperand::BaseDisp { base, disp } => {
                Ok(Address::BaseOffset {
                    base: VReg::Arch(ArchReg::X86(*base)),
                    offset: *disp as i64,
                })
            }
            
            X86MemOperand::Sib { base, index, scale, disp } => {
                Ok(Address::BaseIndexScale {
                    base: base.map(|r| VReg::Arch(ArchReg::X86(r))),
                    index: VReg::Arch(ArchReg::X86(*index)),
                    scale: *scale,
                    disp: *disp,
                })
            }
            
            X86MemOperand::RipRel { disp } => {
                Ok(Address::PcRel {
                    offset: *disp as i64,
                })
            }
            
            X86MemOperand::Absolute { addr } => {
                Ok(Address::Absolute(*addr))
            }
        }
    }
    
    /// Convert x86 condition code to SMIR condition
    fn x86_cond_to_smir(&self, cond: X86Cond) -> Condition {
        match cond {
            X86Cond::O => Condition::Overflow,
            X86Cond::NO => Condition::NoOverflow,
            X86Cond::B => Condition::Ult,
            X86Cond::NB => Condition::Uge,
            X86Cond::E => Condition::Eq,
            X86Cond::NE => Condition::Ne,
            X86Cond::BE => Condition::Ule,
            X86Cond::NBE => Condition::Ugt,
            X86Cond::S => Condition::Negative,
            X86Cond::NS => Condition::Positive,
            X86Cond::P => Condition::Parity,
            X86Cond::NP => Condition::NoParity,
            X86Cond::L => Condition::Slt,
            X86Cond::NL => Condition::Sge,
            X86Cond::LE => Condition::Sle,
            X86Cond::NLE => Condition::Sgt,
        }
    }
}
```

## 4. AArch64 Lifter

```rust
/// AArch64 instruction lifter
pub struct Aarch64Lifter {
    /// Decoder
    decoder: Aarch64Decoder,
}

impl SmirLifter for Aarch64Lifter {
    fn source_arch(&self) -> SourceArch {
        SourceArch::Aarch64
    }
    
    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        if bytes.len() < 4 {
            return Err(LiftError::Incomplete { addr, have: bytes.len(), need: 4 });
        }
        
        let insn_bytes: [u8; 4] = bytes[..4].try_into().unwrap();
        let decoded = self.decoder.decode(insn_bytes)?;
        
        ctx.guest_pc = addr;
        let mut ops = Vec::new();
        let mut branch_targets = Vec::new();
        
        let control_flow = match &decoded {
            // Data processing - register
            Aarch64Insn::AddSubReg { sf, op, s, rm, rn, rd, shift, imm6 } => {
                let width = if *sf { OpWidth::W64 } else { OpWidth::W32 };
                let is_sub = *op;
                let update_flags = *s;
                
                let src1 = VReg::Arch(ArchReg::Arm(ArmReg::X(*rn)));
                let src2 = self.lift_shifted_reg(*rm, *shift, *imm6, width)?;
                let dst = ctx.vreg_alloc.alloc();
                
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: if is_sub {
                        OpKind::Sub { dst, src1, src2, width, flags: if update_flags { FlagUpdate::All } else { FlagUpdate::None } }
                    } else {
                        OpKind::Add { dst, src1, src2, width, flags: if update_flags { FlagUpdate::All } else { FlagUpdate::None } }
                    },
                });
                
                // Write result (handle SP vs ZR)
                self.write_gpr(&mut ops, ctx, *rd, dst, width)?;
                
                ControlFlow::Fallthrough
            }
            
            // Data processing - immediate
            Aarch64Insn::AddSubImm { sf, op, s, sh, imm12, rn, rd } => {
                let width = if *sf { OpWidth::W64 } else { OpWidth::W32 };
                let is_sub = *op;
                let update_flags = *s;
                let imm_val = if *sh { (*imm12 as i64) << 12 } else { *imm12 as i64 };
                
                let src1 = VReg::Arch(ArchReg::Arm(ArmReg::X(*rn)));
                let src2 = SrcOperand::Imm(imm_val);
                let dst = ctx.vreg_alloc.alloc();
                
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: if is_sub {
                        OpKind::Sub { dst, src1, src2, width, flags: if update_flags { FlagUpdate::All } else { FlagUpdate::None } }
                    } else {
                        OpKind::Add { dst, src1, src2, width, flags: if update_flags { FlagUpdate::All } else { FlagUpdate::None } }
                    },
                });
                
                self.write_gpr(&mut ops, ctx, *rd, dst, width)?;
                
                ControlFlow::Fallthrough
            }
            
            // Conditional branch
            Aarch64Insn::BCond { imm19, cond } => {
                let offset = sign_extend_19(*imm19) * 4;
                let target = addr.wrapping_add(offset as u64);
                let smir_cond = self.arm_cond_to_smir(*cond);
                
                branch_targets.push(target);
                branch_targets.push(addr + 4);
                
                ControlFlow::CondBranch {
                    cond: smir_cond,
                    target,
                    fallthrough: addr + 4,
                }
            }
            
            // Unconditional branch
            Aarch64Insn::B { imm26 } => {
                let offset = sign_extend_26(*imm26) * 4;
                let target = addr.wrapping_add(offset as u64);
                
                branch_targets.push(target);
                ControlFlow::Branch { target }
            }
            
            // Branch and link
            Aarch64Insn::BL { imm26 } => {
                let offset = sign_extend_26(*imm26) * 4;
                let target = addr.wrapping_add(offset as u64);
                
                // LR = PC + 4
                let ret_addr = addr + 4;
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                        src: SrcOperand::Imm(ret_addr as i64),
                        width: OpWidth::W64,
                    },
                });
                
                branch_targets.push(target);
                ControlFlow::Call {
                    target: CallTarget::GuestAddr(target),
                }
            }
            
            // Branch to register
            Aarch64Insn::BR { rn } => {
                let target = VReg::Arch(ArchReg::Arm(ArmReg::X(*rn)));
                // Note: need to handle through terminator
                ControlFlow::IndirectBranch
            }
            
            // Branch and link to register
            Aarch64Insn::BLR { rn } => {
                let ret_addr = addr + 4;
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Arm(ArmReg::X(30))),
                        src: SrcOperand::Imm(ret_addr as i64),
                        width: OpWidth::W64,
                    },
                });
                
                let target = VReg::Arch(ArchReg::Arm(ArmReg::X(*rn)));
                ControlFlow::Call {
                    target: CallTarget::Indirect(target),
                }
            }
            
            // Return
            Aarch64Insn::RET { rn } => {
                // Target is typically X30, but can be other register
                ControlFlow::Return
            }
            
            // Load register (unsigned offset)
            Aarch64Insn::LdrUnsignedOff { size, opc, imm12, rn, rt } => {
                let (mem_width, sign) = self.decode_ldr_size(*size, *opc)?;
                let scale = mem_width.bytes() as u64;
                let offset = (*imm12 as u64) * scale;
                
                let addr_op = Address::BaseOffset {
                    base: VReg::Arch(ArchReg::Arm(ArmReg::X(*rn))),
                    offset: offset as i64,
                };
                
                let dst = ctx.vreg_alloc.alloc();
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Load { dst, addr: addr_op, width: mem_width, sign },
                });
                
                self.write_gpr(&mut ops, ctx, *rt, dst, OpWidth::W64)?;
                
                ControlFlow::Fallthrough
            }
            
            // Store register (unsigned offset)
            Aarch64Insn::StrUnsignedOff { size, opc, imm12, rn, rt } => {
                let mem_width = self.decode_str_size(*size)?;
                let scale = mem_width.bytes() as u64;
                let offset = (*imm12 as u64) * scale;
                
                let addr_op = Address::BaseOffset {
                    base: VReg::Arch(ArchReg::Arm(ArmReg::X(*rn))),
                    offset: offset as i64,
                };
                
                let src = VReg::Arch(ArchReg::Arm(ArmReg::X(*rt)));
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Store { src, addr: addr_op, width: mem_width },
                });
                
                ControlFlow::Fallthrough
            }
            
            // SVC (supervisor call)
            Aarch64Insn::SVC { imm16 } => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::ArmSpecific(ArmSpecificOp::Svc { imm: *imm16 }),
                });
                ControlFlow::Syscall
            }
            
            // NOP
            Aarch64Insn::Nop => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Nop,
                });
                ControlFlow::Fallthrough
            }
            
            _ => {
                return Err(LiftError::Unsupported {
                    addr,
                    mnemonic: format!("{:?}", decoded),
                });
            }
        };
        
        Ok(LiftResult {
            ops,
            bytes_consumed: 4,
            control_flow,
            branch_targets,
        })
    }
    
    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError> {
        // Similar to x86, but 4-byte aligned instructions
        // ...
        todo!()
    }
    
    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError> {
        todo!()
    }
}

impl Aarch64Lifter {
    /// Convert ARM condition to SMIR condition
    fn arm_cond_to_smir(&self, cond: u8) -> Condition {
        match cond {
            0b0000 => Condition::Eq,       // EQ
            0b0001 => Condition::Ne,       // NE
            0b0010 => Condition::Uge,      // CS/HS (note: ARM C is inverted)
            0b0011 => Condition::Ult,      // CC/LO
            0b0100 => Condition::Negative, // MI
            0b0101 => Condition::Positive, // PL
            0b0110 => Condition::Overflow, // VS
            0b0111 => Condition::NoOverflow, // VC
            0b1000 => Condition::Ugt,      // HI
            0b1001 => Condition::Ule,      // LS
            0b1010 => Condition::Sge,      // GE
            0b1011 => Condition::Slt,      // LT
            0b1100 => Condition::Sgt,      // GT
            0b1101 => Condition::Sle,      // LE
            0b1110 | 0b1111 => Condition::Always, // AL/NV
            _ => unreachable!(),
        }
    }
    
    /// Lift shifted register operand
    fn lift_shifted_reg(
        &self,
        rm: u8,
        shift_type: u8,
        imm6: u8,
        width: OpWidth,
    ) -> Result<SrcOperand, LiftError> {
        let reg = VReg::Arch(ArchReg::Arm(ArmReg::X(rm)));
        
        if imm6 == 0 {
            return Ok(SrcOperand::Reg(reg));
        }
        
        let shift = match shift_type {
            0b00 => ShiftOp::Lsl,
            0b01 => ShiftOp::Lsr,
            0b10 => ShiftOp::Asr,
            0b11 => ShiftOp::Ror,
            _ => unreachable!(),
        };
        
        Ok(SrcOperand::Shifted {
            reg,
            shift,
            amount: imm6,
        })
    }
    
    /// Write to GPR (handles ZR vs SP distinction)
    fn write_gpr(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        rd: u8,
        src: VReg,
        width: OpWidth,
    ) -> Result<(), LiftError> {
        if rd == 31 {
            // Writing to ZR is a no-op
            return Ok(());
        }
        
        ops.push(SmirOp {
            id: OpId(ops.len() as u16),
            guest_pc: ctx.guest_pc,
            kind: OpKind::Mov {
                dst: VReg::Arch(ArchReg::Arm(ArmReg::X(rd))),
                src: SrcOperand::Reg(src),
                width,
            },
        });
        
        Ok(())
    }
    
    /// Decode LDR size/sign
    fn decode_ldr_size(&self, size: u8, opc: u8) -> Result<(MemWidth, SignExtend), LiftError> {
        let width = match size {
            0b00 => MemWidth::B1,
            0b01 => MemWidth::B2,
            0b10 => MemWidth::B4,
            0b11 => MemWidth::B8,
            _ => unreachable!(),
        };
        let sign = if opc & 1 != 0 { SignExtend::Sign } else { SignExtend::Zero };
        Ok((width, sign))
    }
    
    /// Decode STR size
    fn decode_str_size(&self, size: u8) -> Result<MemWidth, LiftError> {
        Ok(match size {
            0b00 => MemWidth::B1,
            0b01 => MemWidth::B2,
            0b10 => MemWidth::B4,
            0b11 => MemWidth::B8,
            _ => unreachable!(),
        })
    }
}

fn sign_extend_19(val: u32) -> i64 {
    ((val as i32) << 13 >> 13) as i64
}

fn sign_extend_26(val: u32) -> i64 {
    ((val as i32) << 6 >> 6) as i64
}
```

## 5. Hexagon Lifter

```rust
/// Hexagon instruction lifter
pub struct HexagonLifter;

impl SmirLifter for HexagonLifter {
    fn source_arch(&self) -> SourceArch {
        SourceArch::Hexagon
    }
    
    fn lift_insn(
        &mut self,
        addr: GuestAddr,
        bytes: &[u8],
        ctx: &mut LiftContext,
    ) -> Result<LiftResult, LiftError> {
        // Hexagon uses the existing DecodedInsn which is already IR-like
        // This is the simplest lifter - almost 1:1 mapping
        
        if bytes.len() < 4 {
            return Err(LiftError::Incomplete { addr, have: bytes.len(), need: 4 });
        }
        
        let word = u32::from_le_bytes(bytes[..4].try_into().unwrap());
        let decoded = hexagon_decode(word)?;
        
        let mut ops = Vec::new();
        let control_flow = self.lift_hexagon_insn(&mut ops, ctx, &decoded, addr)?;
        
        Ok(LiftResult {
            ops,
            bytes_consumed: 4,
            control_flow,
            branch_targets: vec![],
        })
    }
    
    fn lift_block(
        &mut self,
        addr: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirBlock, LiftError> {
        todo!()
    }
    
    fn lift_function(
        &mut self,
        entry: GuestAddr,
        mem: &dyn MemoryReader,
        ctx: &mut LiftContext,
    ) -> Result<SmirFunction, LiftError> {
        todo!()
    }
}

impl HexagonLifter {
    fn lift_hexagon_insn(
        &self,
        ops: &mut Vec<SmirOp>,
        ctx: &mut LiftContext,
        insn: &HexagonDecodedInsn,
        addr: GuestAddr,
    ) -> Result<ControlFlow, LiftError> {
        match insn {
            HexagonDecodedInsn::Add { dst, src1, src2 } => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Add {
                        dst: self.hex_reg(*dst),
                        src1: self.hex_reg(*src1),
                        src2: SrcOperand::Reg(self.hex_reg(*src2)),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                });
                Ok(ControlFlow::Fallthrough)
            }
            
            HexagonDecodedInsn::AddImm { dst, src, imm } => {
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Add {
                        dst: self.hex_reg(*dst),
                        src1: self.hex_reg(*src),
                        src2: SrcOperand::Imm(*imm as i64),
                        width: OpWidth::W32,
                        flags: FlagUpdate::None,
                    },
                });
                Ok(ControlFlow::Fallthrough)
            }
            
            HexagonDecodedInsn::Load { dst, addr: mem_addr, width, sign, pred: _ } => {
                let mem_width = match width {
                    HexMemWidth::Byte => MemWidth::B1,
                    HexMemWidth::Half => MemWidth::B2,
                    HexMemWidth::Word => MemWidth::B4,
                    HexMemWidth::Double => MemWidth::B8,
                };
                let sign_ext = match sign {
                    HexMemSign::Signed => SignExtend::Sign,
                    HexMemSign::Unsigned => SignExtend::Zero,
                };
                
                let address = self.lift_hex_addr(mem_addr);
                
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Load {
                        dst: self.hex_reg(*dst),
                        addr: address,
                        width: mem_width,
                        sign: sign_ext,
                    },
                });
                Ok(ControlFlow::Fallthrough)
            }
            
            HexagonDecodedInsn::Jump { offset } => {
                let target = addr.wrapping_add(*offset as u64);
                Ok(ControlFlow::Branch { target })
            }
            
            HexagonDecodedInsn::Call { offset } => {
                let target = addr.wrapping_add(*offset as u64);
                // Save return address to LR
                ops.push(SmirOp {
                    id: OpId(ops.len() as u16),
                    guest_pc: addr,
                    kind: OpKind::Mov {
                        dst: VReg::Arch(ArchReg::Hexagon(HexagonReg::Lr)),
                        src: SrcOperand::Imm((addr + 4) as i64),
                        width: OpWidth::W32,
                    },
                });
                Ok(ControlFlow::Call {
                    target: CallTarget::GuestAddr(target),
                })
            }
            
            // ... more Hexagon instructions
            
            _ => Err(LiftError::Unsupported {
                addr,
                mnemonic: format!("{:?}", insn),
            }),
        }
    }
    
    fn hex_reg(&self, reg: u8) -> VReg {
        VReg::Arch(ArchReg::Hexagon(HexagonReg::R(reg)))
    }
    
    fn lift_hex_addr(&self, addr: &HexAddrMode) -> Address {
        match addr {
            HexAddrMode::Offset { base, offset } => Address::BaseOffset {
                base: self.hex_reg(*base),
                offset: *offset as i64,
            },
            HexAddrMode::GpOffset { offset } => Address::GpRel {
                offset: *offset,
            },
            HexAddrMode::Abs { addr } => Address::Absolute(*addr as u64),
            _ => todo!(),
        }
    }
}
```

## 6. Unified Lifter Factory

```rust
/// Create a lifter for the given architecture
pub fn create_lifter(arch: SourceArch) -> Box<dyn SmirLifter> {
    match arch {
        SourceArch::X86_64 => Box::new(X86Lifter::new()),
        SourceArch::Aarch64 => Box::new(Aarch64Lifter::new()),
        SourceArch::Hexagon => Box::new(HexagonLifter),
        _ => panic!("Unsupported architecture: {:?}", arch),
    }
}
```
