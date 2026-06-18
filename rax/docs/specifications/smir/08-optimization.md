# SMIR Optimization Passes

This document specifies optimization passes that can be applied to SMIR before execution.

## 1. Optimization Pipeline

```rust
/// Optimization level
#[derive(Clone, Copy, Debug)]
pub enum OptLevel {
    /// No optimization (for debugging)
    O0,
    
    /// Basic optimizations (fast compile, some speedup)
    O1,
    
    /// Full optimization (slower compile, best runtime)
    O2,
}

/// Run optimization pipeline on a function
pub fn optimize_function(func: &mut SmirFunction, level: OptLevel) {
    match level {
        OptLevel::O0 => {}
        OptLevel::O1 => {
            for block in &mut func.blocks {
                dead_flag_elimination(block);
                constant_propagation(block);
                dead_code_elimination(block);
            }
        }
        OptLevel::O2 => {
            for block in &mut func.blocks {
                dead_flag_elimination(block);
                constant_propagation(block);
                constant_folding(block);
                strength_reduction(block);
                dead_code_elimination(block);
            }
            block_merging(func);
            redundant_load_elimination(func);
        }
    }
}
```

## 2. Dead Flag Elimination

The most impactful optimization for x86 - removes flag updates that are never read.

```rust
/// Eliminate dead flag updates
pub fn dead_flag_elimination(block: &mut SmirBlock) {
    // Backward analysis to find live flags
    let mut live_out = FlagSet::EMPTY;
    
    // Check terminator for flag usage
    if let Terminator::CondBranch { cond, .. } = &block.terminator {
        live_out = live_out.union(cond.required_flags());
    }
    
    // Map from op index to live flags after that op
    let mut liveness = vec![FlagSet::EMPTY; block.ops.len()];
    
    // Backward pass
    for i in (0..block.ops.len()).rev() {
        liveness[i] = live_out;
        
        let op = &block.ops[i];
        let reads = op.kind.flags_read();
        let writes = op.kind.flags_written();
        
        // live_in = (live_out - writes) | reads
        live_out = live_out.difference(writes).union(reads);
    }
    
    // Forward pass: eliminate dead flag updates
    for i in 0..block.ops.len() {
        let live = liveness[i];
        
        if let Some(flags) = block.ops[i].kind.flags_written_mut() {
            if live.intersection(flags.as_set()).is_empty() {
                *flags = FlagUpdate::None;
            }
        }
    }
}

impl FlagSet {
    /// Set difference
    pub fn difference(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 & !other.0)
    }
    
    /// Set intersection
    pub fn intersection(self, other: FlagSet) -> FlagSet {
        FlagSet(self.0 & other.0)
    }
    
    /// Check if empty
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl OpKind {
    /// Get mutable reference to flag update field
    pub fn flags_written_mut(&mut self) -> Option<&mut FlagUpdate> {
        match self {
            OpKind::Add { flags, .. } |
            OpKind::Sub { flags, .. } |
            OpKind::Adc { flags, .. } |
            OpKind::Sbb { flags, .. } |
            OpKind::And { flags, .. } |
            OpKind::Or { flags, .. } |
            OpKind::Xor { flags, .. } |
            OpKind::Shl { flags, .. } |
            OpKind::Shr { flags, .. } |
            OpKind::Sar { flags, .. } => Some(flags),
            _ => None,
        }
    }
}

impl FlagUpdate {
    pub fn as_set(&self) -> FlagSet {
        match self {
            FlagUpdate::None => FlagSet::EMPTY,
            FlagUpdate::All => FlagSet::ALL_X86,
            FlagUpdate::Specific(set) => *set,
        }
    }
}
```

## 3. Constant Propagation

Track known constant values through the block.

```rust
/// Constant propagation within a block
pub fn constant_propagation(block: &mut SmirBlock) {
    let mut constants: HashMap<VReg, i64> = HashMap::new();
    
    for op in &mut block.ops {
        match &mut op.kind {
            OpKind::Mov { dst, src, width } => {
                if let SrcOperand::Imm(imm) = src {
                    constants.insert(*dst, *imm);
                } else if let SrcOperand::Reg(r) = src {
                    if let Some(&val) = constants.get(r) {
                        *src = SrcOperand::Imm(val);
                        constants.insert(*dst, val);
                    } else {
                        constants.remove(dst);
                    }
                }
            }
            
            OpKind::Add { dst, src1, src2, .. } => {
                // Try to replace src2 with constant if known
                if let SrcOperand::Reg(r) = src2 {
                    if let Some(&val) = constants.get(r) {
                        *src2 = SrcOperand::Imm(val);
                    }
                }
                
                // Check if src1 is also constant
                if let Some(&v1) = constants.get(src1) {
                    if let SrcOperand::Imm(v2) = src2 {
                        constants.insert(*dst, v1.wrapping_add(*v2));
                    } else {
                        constants.remove(dst);
                    }
                } else {
                    constants.remove(dst);
                }
            }
            
            OpKind::Sub { dst, src1, src2, .. } => {
                if let SrcOperand::Reg(r) = src2 {
                    if let Some(&val) = constants.get(r) {
                        *src2 = SrcOperand::Imm(val);
                    }
                }
                
                if let Some(&v1) = constants.get(src1) {
                    if let SrcOperand::Imm(v2) = src2 {
                        constants.insert(*dst, v1.wrapping_sub(*v2));
                    } else {
                        constants.remove(dst);
                    }
                } else {
                    constants.remove(dst);
                }
            }
            
            OpKind::And { dst, src1, src2, .. } => {
                if let SrcOperand::Reg(r) = src2 {
                    if let Some(&val) = constants.get(r) {
                        *src2 = SrcOperand::Imm(val);
                    }
                }
                constants.remove(dst);
            }
            
            OpKind::Load { dst, .. } |
            OpKind::AtomicLoad { dst, .. } => {
                // Loads produce unknown values
                constants.remove(dst);
            }
            
            _ => {
                // For other ops, invalidate destination if it exists
                if let Some(dst) = op.kind.dest_vreg() {
                    constants.remove(&dst);
                }
            }
        }
    }
}

impl OpKind {
    /// Get destination register if any
    pub fn dest_vreg(&self) -> Option<VReg> {
        match self {
            OpKind::Add { dst, .. } |
            OpKind::Sub { dst, .. } |
            OpKind::And { dst, .. } |
            OpKind::Or { dst, .. } |
            OpKind::Xor { dst, .. } |
            OpKind::Mov { dst, .. } |
            OpKind::Load { dst, .. } |
            OpKind::Shl { dst, .. } |
            OpKind::Shr { dst, .. } |
            OpKind::ZeroExtend { dst, .. } |
            OpKind::SignExtend { dst, .. } => Some(*dst),
            _ => None,
        }
    }
}
```

## 4. Constant Folding

Evaluate constant expressions at compile time.

```rust
/// Fold constant expressions
pub fn constant_folding(block: &mut SmirBlock) {
    let mut i = 0;
    while i < block.ops.len() {
        let folded = match &block.ops[i].kind {
            OpKind::Add { dst, src1, src2, width, .. } => {
                if let (VReg::Imm(v1), SrcOperand::Imm(v2)) = (src1, src2) {
                    let result = ((*v1 as u64).wrapping_add(*v2 as u64)) & width.mask();
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(result as i64),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            OpKind::Sub { dst, src1, src2, width, .. } => {
                if let (VReg::Imm(v1), SrcOperand::Imm(v2)) = (src1, src2) {
                    let result = ((*v1 as u64).wrapping_sub(*v2 as u64)) & width.mask();
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(result as i64),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            OpKind::And { dst, src1, src2, width, .. } => {
                if let (VReg::Imm(v1), SrcOperand::Imm(v2)) = (src1, src2) {
                    let result = (*v1 & *v2) as u64 & width.mask();
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(result as i64),
                        width: *width,
                    })
                } else if let SrcOperand::Imm(0) = src2 {
                    // x & 0 = 0
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(0),
                        width: *width,
                    })
                } else if let SrcOperand::Imm(-1) = src2 {
                    // x & -1 = x
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Reg(*src1),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            OpKind::Or { dst, src1, src2, width, .. } => {
                if let SrcOperand::Imm(0) = src2 {
                    // x | 0 = x
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Reg(*src1),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            OpKind::Xor { dst, src1, src2, width, .. } => {
                if src1 == &SrcOperand::Reg(*src1).unwrap_reg() && 
                   matches!(src2, SrcOperand::Reg(r) if r == src1) {
                    // x ^ x = 0
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Imm(0),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            OpKind::Shl { dst, src, amount, width, .. } => {
                if let SrcOperand::Imm(0) = amount {
                    // x << 0 = x
                    Some(OpKind::Mov {
                        dst: *dst,
                        src: SrcOperand::Reg(*src),
                        width: *width,
                    })
                } else {
                    None
                }
            }
            
            _ => None,
        };
        
        if let Some(new_kind) = folded {
            block.ops[i].kind = new_kind;
        }
        
        i += 1;
    }
}
```

## 5. Dead Code Elimination

Remove operations whose results are never used.

```rust
/// Eliminate dead code
pub fn dead_code_elimination(block: &mut SmirBlock) {
    // Build use set
    let mut used: HashSet<VReg> = HashSet::new();
    
    // Terminator uses
    match &block.terminator {
        Terminator::CondBranch { cond, .. } => {
            used.insert(*cond);
        }
        Terminator::IndirectBranch { target, .. } => {
            used.insert(*target);
        }
        Terminator::Return { values } => {
            for v in values {
                used.insert(*v);
            }
        }
        _ => {}
    }
    
    // Backward pass to find all used values
    for op in block.ops.iter().rev() {
        // If this op's result is used, mark its sources as used
        let is_used = op.kind.dest_vreg().map_or(true, |d| used.contains(&d));
        
        if is_used || op.kind.has_side_effects() {
            for src in op.kind.source_vregs() {
                used.insert(src);
            }
        }
    }
    
    // Remove unused ops
    block.ops.retain(|op| {
        op.kind.dest_vreg().map_or(true, |d| used.contains(&d)) ||
        op.kind.has_side_effects()
    });
}

impl OpKind {
    /// Get source registers
    pub fn source_vregs(&self) -> Vec<VReg> {
        let mut result = Vec::new();
        
        match self {
            OpKind::Add { src1, src2, .. } |
            OpKind::Sub { src1, src2, .. } |
            OpKind::And { src1, src2, .. } |
            OpKind::Or { src1, src2, .. } |
            OpKind::Xor { src1, src2, .. } => {
                result.push(*src1);
                if let SrcOperand::Reg(r) = src2 {
                    result.push(*r);
                }
            }
            
            OpKind::Mov { src, .. } => {
                if let SrcOperand::Reg(r) = src {
                    result.push(*r);
                }
            }
            
            OpKind::Load { addr, .. } |
            OpKind::Store { addr, .. } => {
                result.extend(self.address_vregs(addr));
            }
            
            _ => {}
        }
        
        result
    }
    
    /// Check if op has side effects (cannot be eliminated even if result unused)
    pub fn has_side_effects(&self) -> bool {
        matches!(self,
            OpKind::Store { .. } |
            OpKind::AtomicStore { .. } |
            OpKind::AtomicRmw { .. } |
            OpKind::Cas { .. } |
            OpKind::StoreExclusive { .. } |
            OpKind::Fence { .. } |
            OpKind::Syscall { .. } |
            OpKind::X86Specific(_) |
            OpKind::ArmSpecific(_) |
            OpKind::HexagonSpecific(_)
        )
    }
    
    fn address_vregs(&self, addr: &Address) -> Vec<VReg> {
        match addr {
            Address::Direct(r) => vec![*r],
            Address::BaseOffset { base, .. } => vec![*base],
            Address::BaseIndexScale { base, index, .. } => {
                let mut v = vec![*index];
                if let Some(b) = base {
                    v.push(*b);
                }
                v
            }
            _ => vec![],
        }
    }
}
```

## 6. Strength Reduction

Replace expensive operations with cheaper equivalents.

```rust
/// Strength reduction transformations
pub fn strength_reduction(block: &mut SmirBlock) {
    for op in &mut block.ops {
        match &mut op.kind {
            // Multiply by power of 2 -> shift
            OpKind::MulU { dst, src1, src2, width, .. } |
            OpKind::MulS { dst, src1, src2, width, .. } => {
                if let SrcOperand::Imm(imm) = src2 {
                    if *imm > 0 && (*imm as u64).is_power_of_two() {
                        let shift = (*imm as u64).trailing_zeros() as u8;
                        op.kind = OpKind::Shl {
                            dst: *dst,
                            src: *src1,
                            amount: SrcOperand::Imm(shift as i64),
                            width: *width,
                            flags: FlagUpdate::None,
                        };
                    }
                }
            }
            
            // Divide by power of 2 -> shift (unsigned only)
            OpKind::DivU { quot, src1, src2, width, .. } => {
                if let SrcOperand::Imm(imm) = src2 {
                    if *imm > 0 && (*imm as u64).is_power_of_two() {
                        let shift = (*imm as u64).trailing_zeros() as u8;
                        op.kind = OpKind::Shr {
                            dst: *quot,
                            src: *src1,
                            amount: SrcOperand::Imm(shift as i64),
                            width: *width,
                            flags: FlagUpdate::None,
                        };
                    }
                }
            }
            
            // Add 1 -> Inc (could be useful for some backends)
            // Sub 1 -> Dec
            
            _ => {}
        }
    }
}
```

## 7. Block Merging

Merge blocks with unconditional branches.

```rust
/// Merge adjacent blocks with unconditional jumps
pub fn block_merging(func: &mut SmirFunction) {
    // Build predecessor count
    let mut pred_count: HashMap<BlockId, usize> = HashMap::new();
    
    for block in &func.blocks {
        match &block.terminator {
            Terminator::Branch { target } => {
                *pred_count.entry(*target).or_default() += 1;
            }
            Terminator::CondBranch { true_target, false_target, .. } => {
                *pred_count.entry(*true_target).or_default() += 1;
                *pred_count.entry(*false_target).or_default() += 1;
            }
            _ => {}
        }
    }
    
    // Find blocks with single predecessor that is unconditional branch
    let mut merge_pairs: Vec<(BlockId, BlockId)> = Vec::new();
    
    for block in &func.blocks {
        if let Terminator::Branch { target } = &block.terminator {
            if pred_count.get(target) == Some(&1) {
                merge_pairs.push((block.id, *target));
            }
        }
    }
    
    // Merge blocks (simplified - would need proper implementation)
    for (from, to) in merge_pairs {
        if let Some(from_idx) = func.blocks.iter().position(|b| b.id == from) {
            if let Some(to_idx) = func.blocks.iter().position(|b| b.id == to) {
                // Append ops from 'to' block to 'from' block
                let to_ops = func.blocks[to_idx].ops.clone();
                let to_term = func.blocks[to_idx].terminator.clone();
                
                func.blocks[from_idx].ops.extend(to_ops);
                func.blocks[from_idx].terminator = to_term;
                
                // Mark 'to' block for removal
                func.blocks[to_idx].ops.clear();
            }
        }
    }
    
    // Remove empty blocks
    func.blocks.retain(|b| !b.ops.is_empty() || b.id == func.entry);
}
```

## 8. Redundant Load Elimination

Remove loads that are already in a register.

```rust
/// Eliminate redundant loads
pub fn redundant_load_elimination(func: &mut SmirFunction) {
    for block in &mut func.blocks {
        // Track what's currently in registers from memory
        let mut mem_to_reg: HashMap<(Address, MemWidth), VReg> = HashMap::new();
        
        let mut new_ops = Vec::new();
        
        for op in &block.ops {
            match &op.kind {
                OpKind::Load { dst, addr, width, .. } => {
                    let key = (addr.clone(), *width);
                    if let Some(&existing) = mem_to_reg.get(&key) {
                        // Replace load with move
                        new_ops.push(SmirOp {
                            id: op.id,
                            guest_pc: op.guest_pc,
                            kind: OpKind::Mov {
                                dst: *dst,
                                src: SrcOperand::Reg(existing),
                                width: OpWidth::W64,
                            },
                        });
                    } else {
                        mem_to_reg.insert(key, *dst);
                        new_ops.push(op.clone());
                    }
                }
                
                OpKind::Store { addr, width, .. } => {
                    // Invalidate any loads from this address
                    mem_to_reg.remove(&(addr.clone(), *width));
                    new_ops.push(op.clone());
                }
                
                _ => {
                    new_ops.push(op.clone());
                }
            }
        }
        
        block.ops = new_ops;
    }
}
```

## 9. Optimization Statistics

```rust
/// Statistics from optimization passes
#[derive(Default, Debug)]
pub struct OptStats {
    /// Dead flag updates eliminated
    pub dead_flags_eliminated: usize,
    
    /// Constants propagated
    pub constants_propagated: usize,
    
    /// Expressions folded
    pub expressions_folded: usize,
    
    /// Dead ops eliminated
    pub dead_ops_eliminated: usize,
    
    /// Strength reductions applied
    pub strength_reductions: usize,
    
    /// Blocks merged
    pub blocks_merged: usize,
    
    /// Redundant loads eliminated
    pub redundant_loads_eliminated: usize,
}
```

## 10. Future Optimizations

### 10.1 Loop Optimizations
- Loop invariant code motion
- Loop unrolling
- Induction variable simplification

### 10.2 Inter-Block Optimizations
- Value numbering
- Common subexpression elimination
- Partial redundancy elimination

### 10.3 Register Allocation
- Linear scan allocation for JIT
- Graph coloring for AOT
- Spill code generation

### 10.4 Instruction Scheduling
- Basic block scheduling
- Software pipelining
- Critical path optimization
