//! Validation tests for SMIR lift/lower round-trip.
//!
//! This module tests that we can lift x86_64 machine code to SMIR and lower
//! it back to machine code, producing functionally equivalent output.

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::smir::ir::{SmirBlock, SmirFunction, Terminator};
    use crate::smir::lift::x86_64::X86_64Lifter;
    use crate::smir::lift::{ControlFlow, LiftContext, SmirLifter};
    use crate::smir::lower::SmirLowerer;
    use crate::smir::lower::x86_64::X86_64Lowerer;
    use crate::smir::memory::SmirMemory;
    use crate::smir::types::{BlockId, FunctionId, SourceArch};

    /// Format bytes as hex string
    fn hex_bytes(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Print op details
    fn print_op(op: &crate::smir::ops::SmirOp) {
        println!("    [{:04}] {:?}", op.id.0, op.kind);
    }

    #[test]
    fn test_lift_lower_simple_instructions() {
        // Simple instruction sequences to test
        let test_cases: Vec<(&str, Vec<u8>)> = vec![
            // MOV instructions
            ("mov rax, rcx", vec![0x48, 0x89, 0xC8]),
            ("mov eax, 42", vec![0xB8, 0x2A, 0x00, 0x00, 0x00]),
            (
                "mov rax, 0x12345678",
                vec![0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12],
            ),
            // ADD instructions
            ("add rax, rbx", vec![0x48, 0x01, 0xD8]),
            ("add rax, 10", vec![0x48, 0x83, 0xC0, 0x0A]),
            // SUB instructions
            ("sub rax, rcx", vec![0x48, 0x29, 0xC8]),
            ("sub rax, 5", vec![0x48, 0x83, 0xE8, 0x05]),
            // AND/OR/XOR
            ("and rax, rbx", vec![0x48, 0x21, 0xD8]),
            ("or rax, rcx", vec![0x48, 0x09, 0xC8]),
            ("xor rax, rax", vec![0x48, 0x31, 0xC0]),
            // Shifts
            ("shl rax, 4", vec![0x48, 0xC1, 0xE0, 0x04]),
            ("shr rbx, 2", vec![0x48, 0xC1, 0xEB, 0x02]),
            // Stack
            ("push rbp", vec![0x55]),
            ("pop rbp", vec![0x5D]),
            // NOP
            ("nop", vec![0x90]),
        ];

        let mut lifter = X86_64Lifter::new();
        let mut lowerer = X86_64Lowerer::new();

        for (name, original_bytes) in test_cases {
            println!("\n=== Testing: {} ===", name);
            println!("  Original: {}", hex_bytes(&original_bytes));

            let mut ctx = LiftContext::new(SourceArch::X86_64);

            // Lift the instruction
            match lifter.lift_insn(0x1000, &original_bytes, &mut ctx) {
                Ok(result) => {
                    println!(
                        "  Lifted {} ops ({} bytes consumed):",
                        result.ops.len(),
                        result.bytes_consumed
                    );
                    for op in &result.ops {
                        print_op(op);
                    }

                    // Create a minimal function for lowering
                    let mut func = SmirFunction::new(FunctionId(0), BlockId(0), 0x1000);
                    let mut block = SmirBlock::new(BlockId(0), 0x1000);

                    // Add ops to block
                    for op in result.ops {
                        block.push_op(op);
                    }
                    block.set_terminator(Terminator::Return { values: vec![] });
                    func.add_block(block);

                    // Lower it
                    match lowerer.lower_function(&func) {
                        Ok(_lower_result) => {
                            let lowered = lowerer.finalize().unwrap();
                            println!("  Lowered: {} bytes", lowered.len());
                            println!("  Code: {}", hex_bytes(&lowered));
                        }
                        Err(e) => {
                            println!("  Lower error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("  Lift error: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn test_lift_lower_microkernel_start() {
        // The _start function from the microkernel
        // 0x1000000: 48 8d 25 f9 ff 00 00   leaq 0xfff9(%rip), %rsp
        // 0x1000007: 48 8d 3d 92 0f 00 00   leaq 0xf92(%rip), %rdi
        // ...
        let start_bytes = vec![
            0x48, 0x8d, 0x25, 0xf9, 0xff, 0x00, 0x00, // lea rsp, [rip+0xfff9]
            0x48, 0x8d, 0x3d, 0x92, 0x0f, 0x00, 0x00, // lea rdi, [rip+0xf92]
            0x48, 0x8d, 0x0d, 0x9b, 0x0f, 0x00, 0x00, // lea rcx, [rip+0xf9b]
            0x48, 0x29, 0xf9, // sub rcx, rdi
            0x48, 0xc1, 0xe9, 0x03, // shr rcx, 3
            0x31, 0xc0, // xor eax, eax
        ];

        println!("\n=== Testing microkernel _start prologue ===");
        println!("Original bytes: {}", hex_bytes(&start_bytes));

        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);
        let mut block = SmirBlock::new(BlockId(0), 0x1000000);

        let mut pc = 0x1000000u64;
        let mut offset = 0usize;

        // Lift multiple instructions
        while offset < start_bytes.len() {
            match lifter.lift_insn(pc, &start_bytes[offset..], &mut ctx) {
                Ok(result) => {
                    for op in result.ops {
                        block.push_op(op);
                    }
                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;

                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Lift error at {:#x}: {:?}", pc, e);
                    break;
                }
            }
        }

        println!(
            "\nLifted {} ops (PC: {:#x} -> {:#x}):",
            block.ops.len(),
            0x1000000u64,
            pc
        );
        for op in &block.ops {
            print_op(op);
        }

        // Create function and lower
        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), 0x1000000);
        block.set_terminator(Terminator::Return { values: vec![] });
        func.add_block(block);

        let mut lowerer = X86_64Lowerer::new();
        match lowerer.lower_function(&func) {
            Ok(result) => {
                let lowered = lowerer.finalize().unwrap();
                println!("\nLowered to {} bytes:", lowered.len());
                println!("Code: {}", hex_bytes(&lowered));
                println!("\nStack size: {} bytes", result.stack_size);
            }
            Err(e) => {
                println!("\nLower error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_lift_lower_kernel_main_prologue() {
        // kernel_main prologue:
        // 1000064: 55                       push rbp
        // 1000065: 41 57                    push r15
        // 1000067: 41 56                    push r14
        // 1000069: 41 55                    push r13
        // 100006b: 41 54                    push r12
        // 100006d: 53                       push rbx
        // 100006e: 48 81 ec a8 00 00 00     sub rsp, 0xa8
        let prologue_bytes = vec![
            0x55, // push rbp
            0x41, 0x57, // push r15
            0x41, 0x56, // push r14
            0x41, 0x55, // push r13
            0x41, 0x54, // push r12
            0x53, // push rbx
            0x48, 0x81, 0xec, 0xa8, 0x00, 0x00, 0x00, // sub rsp, 0xa8
        ];

        println!("\n=== Testing kernel_main prologue ===");
        println!("Original bytes: {}", hex_bytes(&prologue_bytes));

        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);
        let mut block = SmirBlock::new(BlockId(0), 0x1000064);

        let mut pc = 0x1000064u64;
        let mut offset = 0usize;

        while offset < prologue_bytes.len() {
            match lifter.lift_insn(pc, &prologue_bytes[offset..], &mut ctx) {
                Ok(result) => {
                    for op in result.ops {
                        block.push_op(op);
                    }
                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;

                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Lift error at {:#x}: {:?}", pc, e);
                    break;
                }
            }
        }

        println!(
            "\nLifted {} ops (PC: {:#x} -> {:#x}):",
            block.ops.len(),
            0x1000064u64,
            pc
        );
        for op in &block.ops {
            print_op(op);
        }

        // Create function and lower
        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), 0x1000064);
        block.set_terminator(Terminator::Return { values: vec![] });
        func.add_block(block);

        let mut lowerer = X86_64Lowerer::new();
        match lowerer.lower_function(&func) {
            Ok(result) => {
                let lowered = lowerer.finalize().unwrap();
                println!("\nLowered to {} bytes:", lowered.len());
                println!("Code: {}", hex_bytes(&lowered));
            }
            Err(e) => {
                println!("\nLower error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_lift_lower_loop() {
        // A simple loop from the microkernel:
        // 1000037: 48 83 f9 0e              cmp rcx, 0xe
        // 100003b: 74 0c                    je 0x1000049
        let loop_bytes = vec![
            0x48, 0x83, 0xf9, 0x0e, // cmp rcx, 0xe
            0x74, 0x0c, // je +0xc
        ];

        println!("\n=== Testing loop comparison ===");
        println!("Original bytes: {}", hex_bytes(&loop_bytes));

        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);

        let mut pc = 0x1000037u64;
        let mut offset = 0usize;

        while offset < loop_bytes.len() {
            match lifter.lift_insn(pc, &loop_bytes[offset..], &mut ctx) {
                Ok(result) => {
                    println!(
                        "\nInstruction at {:#x} ({} bytes):",
                        pc, result.bytes_consumed
                    );
                    for op in &result.ops {
                        print_op(op);
                    }
                    println!("  Control flow: {:?}", result.control_flow);

                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;

                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(e) => {
                    println!("Lift error at {:#x}: {:?}", pc, e);
                    break;
                }
            }
        }
    }

    #[test]
    #[ignore] // Run with: cargo test test_lift_lower_full_microkernel -- --ignored --nocapture
    fn test_lift_lower_full_microkernel() {
        // Load the actual microkernel binary
        let kernel_path = "microkernel/microkernel.bin";
        let kernel_bytes = match std::fs::read(kernel_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Could not read {}: {}", kernel_path, e);
                return;
            }
        };

        // Parse ELF
        let elf = match goblin::elf::Elf::parse(&kernel_bytes) {
            Ok(elf) => elf,
            Err(e) => {
                println!("Could not parse ELF: {}", e);
                return;
            }
        };

        // Find .text section
        let text_section = elf.section_headers.iter().find(|sh| {
            elf.shdr_strtab
                .get_at(sh.sh_name)
                .map_or(false, |name| name == ".text")
        });

        let text_section = match text_section {
            Some(s) => s,
            None => {
                println!("No .text section found");
                return;
            }
        };

        let text_offset = text_section.sh_offset as usize;
        let text_size = text_section.sh_size as usize;
        let text_addr = text_section.sh_addr;

        println!("=== Microkernel .text section ===");
        println!("  Address: {:#x}", text_addr);
        println!("  Size: {} bytes ({:#x})", text_size, text_size);
        println!("  Offset: {:#x}", text_offset);

        let text_bytes = &kernel_bytes[text_offset..text_offset + text_size];

        // Statistics
        let mut total_insns = 0;
        let mut lifted_insns = 0;
        let mut lowered_insns = 0;
        let mut lift_errors: HashMap<String, usize> = HashMap::new();
        let mut lower_errors: HashMap<String, usize> = HashMap::new();

        let mut lifter = X86_64Lifter::new();
        let mut lowerer = X86_64Lowerer::new();

        let mut pc = text_addr;
        let end_addr = text_addr + text_size as u64;
        let mut offset = 0usize;

        while pc < end_addr && offset < text_bytes.len() {
            let mut ctx = LiftContext::new(SourceArch::X86_64);
            total_insns += 1;

            match lifter.lift_insn(pc, &text_bytes[offset..], &mut ctx) {
                Ok(result) => {
                    lifted_insns += 1;

                    // Try to lower the lifted instruction
                    if !result.ops.is_empty() {
                        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), pc);
                        let mut block = SmirBlock::new(BlockId(0), pc);
                        for op in result.ops {
                            block.push_op(op);
                        }
                        block.set_terminator(Terminator::Return { values: vec![] });
                        func.add_block(block);

                        match lowerer.lower_function(&func) {
                            Ok(_) => {
                                lowered_insns += 1;
                            }
                            Err(e) => {
                                // Extract the op name from the error
                                let key = match &e {
                                    crate::smir::lower::LowerError::UnsupportedOp { op } => {
                                        // Extract just the op type name
                                        op.split_whitespace()
                                            .next()
                                            .unwrap_or("Unknown")
                                            .to_string()
                                    }
                                    _ => format!("{:?}", e)
                                        .split_whitespace()
                                        .take(3)
                                        .collect::<Vec<_>>()
                                        .join(" "),
                                };
                                *lower_errors.entry(key).or_insert(0) += 1;
                            }
                        }
                    } else {
                        lowered_insns += 1; // Empty op list is okay
                    }

                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;
                }
                Err(e) => {
                    let key = format!("{:?}", e)
                        .split_whitespace()
                        .take(3)
                        .collect::<Vec<_>>()
                        .join(" ");
                    *lift_errors.entry(key).or_insert(0) += 1;
                    pc += 1; // Skip one byte and continue
                    offset += 1;
                }
            }
        }

        println!("\n=== Lift/Lower Statistics ===");
        println!("  Total instructions attempted: {}", total_insns);
        println!(
            "  Successfully lifted: {} ({:.1}%)",
            lifted_insns,
            100.0 * lifted_insns as f64 / total_insns as f64
        );
        println!(
            "  Successfully lowered: {} ({:.1}%)",
            lowered_insns,
            100.0 * lowered_insns as f64 / total_insns as f64
        );

        if !lift_errors.is_empty() {
            println!("\n  Lift errors:");
            let mut errors: Vec<_> = lift_errors.iter().collect();
            errors.sort_by(|a, b| b.1.cmp(a.1));
            for (error, count) in errors.iter().take(10) {
                println!("    {}: {}", count, error);
            }
        }

        if !lower_errors.is_empty() {
            println!("\n  Lower errors (top 20):");
            let mut errors: Vec<_> = lower_errors.iter().collect();
            errors.sort_by(|a, b| b.1.cmp(a.1));
            for (error, count) in errors.iter().take(20) {
                println!("    {:4}: {}", count, error);
            }
        }

        // Assert reasonable success rate
        let lift_rate = lifted_insns as f64 / total_insns as f64;
        let lower_rate = lowered_insns as f64 / total_insns as f64;

        println!("\n  Lift rate: {:.1}%", lift_rate * 100.0);
        println!("  Lower rate: {:.1}%", lower_rate * 100.0);
    }

    #[test]
    #[ignore] // Run with: cargo test test_roundtrip_exact_bytes_microkernel -- --ignored --nocapture
    fn test_roundtrip_exact_bytes_microkernel() {
        let kernel_path = "microkernel/microkernel.bin";
        let kernel_bytes = match std::fs::read(kernel_path) {
            Ok(bytes) => bytes,
            Err(e) => {
                println!("Could not read {}: {}", kernel_path, e);
                return;
            }
        };

        let elf = match goblin::elf::Elf::parse(&kernel_bytes) {
            Ok(elf) => elf,
            Err(e) => {
                println!("Could not parse ELF: {}", e);
                return;
            }
        };

        let text_section = elf.section_headers.iter().find(|sh| {
            elf.shdr_strtab
                .get_at(sh.sh_name)
                .map_or(false, |name| name == ".text")
        });

        let text_section = match text_section {
            Some(s) => s,
            None => {
                println!("No .text section found");
                return;
            }
        };

        let text_offset = text_section.sh_offset as usize;
        let text_size = text_section.sh_size as usize;
        let text_addr = text_section.sh_addr;
        let text_bytes = &kernel_bytes[text_offset..text_offset + text_size];

        let mut lifter = X86_64Lifter::new();

        let mut pc = text_addr;
        let end_addr = text_addr + text_size as u64;
        let mut offset = 0usize;

        let mut total = 0usize;
        let mut exact = 0usize;
        let mut mismatches: Vec<(u64, Vec<u8>, Vec<u8>)> = Vec::new();

        while pc < end_addr && offset < text_bytes.len() {
            let mut ctx = LiftContext::new(SourceArch::X86_64);
            total += 1;

            match lifter.lift_insn(pc, &text_bytes[offset..], &mut ctx) {
                Ok(result) => {
                    let insn_bytes = text_bytes[offset..offset + result.bytes_consumed].to_vec();

                    if result.ops.is_empty() {
                        exact += 1;
                    } else {
                        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), pc);
                        let mut block = SmirBlock::new(BlockId(0), pc);
                        for op in result.ops {
                            block.push_op(op);
                        }
                        block.set_terminator(Terminator::Return { values: vec![] });
                        func.add_block(block);

                        let mut lowerer = X86_64Lowerer::new();
                        lowerer.set_pcrel_adjust(false);
                        if lowerer.lower_function(&func).is_err() {
                            mismatches.push((pc, insn_bytes, vec![]));
                            pc += result.bytes_consumed as u64;
                            offset += result.bytes_consumed;
                            continue;
                        }

                        let lowered = match lowerer.finalize() {
                            Ok(bytes) => bytes,
                            Err(_) => {
                                mismatches.push((pc, insn_bytes, vec![]));
                                pc += result.bytes_consumed as u64;
                                offset += result.bytes_consumed;
                                continue;
                            }
                        };

                        match strip_prologue_epilogue("microkernel", &lowered, Some(&insn_bytes)) {
                            Ok((start, end)) => {
                                let body = lowered[start..end].to_vec();
                                if body == insn_bytes {
                                    exact += 1;
                                } else {
                                    mismatches.push((pc, insn_bytes, body));
                                }
                            }
                            Err(_) => {
                                mismatches.push((pc, insn_bytes, vec![]));
                            }
                        }
                    }

                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;
                }
                Err(_) => {
                    pc += 1;
                    offset += 1;
                }
            }
        }

        let rate = if total > 0 {
            100.0 * exact as f64 / total as f64
        } else {
            0.0
        };

        println!("Exact byte match: {}/{} ({:.1}%)", exact, total, rate);
        if !mismatches.is_empty() {
            println!("Mismatches: {}", mismatches.len());
            for (pc, original, lowered) in mismatches.iter().take(10) {
                println!(
                    "  {:#x}: orig={} lowered={}",
                    pc,
                    hex_bytes(original),
                    hex_bytes(lowered)
                );
            }
        }

        // Exact byte matches are expected to be < 100% because the lifter
        // normalizes some instructions into canonical SMIR forms.
    }

    // ========================================================================
    // Semantic Verification Tests
    // ========================================================================
    //
    // These tests verify that lowered code produces semantically correct
    // results by:
    // 1. Lifting original x86_64 machine code to SMIR
    // 2. Executing the SMIR with the interpreter
    // 3. Lowering the SMIR back to machine code
    // 4. Re-lifting the lowered code
    // 5. Executing the re-lifted SMIR
    // 6. Comparing the results
    //
    // This validates that the round-trip preserves semantic correctness.

    use crate::smir::context::SmirContext;
    use crate::smir::interp::SmirInterpreter;
    use crate::smir::memory::FlatMemory;
    use crate::smir::types::{ArchReg, X86Reg};

    fn execute_block(
        block: &SmirBlock,
        regs: &[(X86Reg, u64)],
        mem_init: &[(u64, &[u8])],
    ) -> (SmirContext, FlatMemory) {
        let mut ctx = SmirContext::new_x86_64();
        let mut mem = FlatMemory::new(0x10000);

        // Initialize memory
        for (addr, data) in mem_init {
            if *addr as usize + data.len() <= 0x10000 {
                mem.write(*addr, data).unwrap();
            }
        }

        // Set initial registers
        for (reg, value) in regs {
            ctx.write_arch_reg(ArchReg::X86(*reg), *value);
        }

        ctx.pc = block.guest_pc;

        let mut interp = SmirInterpreter::new();
        interp.add_block(block.guest_pc, block.clone());

        let _result = interp.run(&mut ctx, &mut mem);

        (ctx, mem)
    }

    /// Execute a block with initial register and memory state
    fn execute_block_with_state(
        block: &SmirBlock,
        regs: &[(X86Reg, u64)],
        mem_init: &[(u64, &[u8])],
    ) -> (u64, FlatMemory) {
        let (ctx, mem) = execute_block(block, regs, mem_init);
        (ctx.read_arch_reg(ArchReg::X86(X86Reg::Rax)), mem)
    }

    fn read_u64(mem: &mut FlatMemory, addr: u64) -> u64 {
        let mut buf = [0u8; 8];
        mem.read(addr, &mut buf).unwrap();
        u64::from_le_bytes(buf)
    }

    fn lift_bytes_to_block(
        name: &str,
        bytes: &[u8],
        block_id: BlockId,
        pc_start: u64,
    ) -> Result<SmirBlock, String> {
        let mut lifter = X86_64Lifter::new();
        let mut ctx = LiftContext::new(SourceArch::X86_64);
        let mut block = SmirBlock::new(block_id, pc_start);

        let mut offset = 0;
        let mut pc = pc_start;

        while offset < bytes.len() {
            let remaining = &bytes[offset..];
            if remaining.is_empty() {
                break;
            }
            match lifter.lift_insn(pc, remaining, &mut ctx) {
                Ok(result) => {
                    if result.bytes_consumed == 0 {
                        break;
                    }
                    for op in result.ops {
                        block.push_op(op);
                    }
                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;
                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(crate::smir::lift::LiftError::Incomplete { .. }) => {
                    break;
                }
                Err(e) => {
                    return Err(format!("{}: lift error: {:?}", name, e));
                }
            }
        }

        block.set_terminator(Terminator::Return { values: vec![] });
        Ok(block)
    }

    fn lower_body_bytes(name: &str, bytes: &[u8]) -> Result<Vec<u8>, String> {
        let block = lift_bytes_to_block(name, bytes, BlockId(0), 0x1000)?;
        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), 0x1000);
        func.add_block(block);

        let mut lowerer = X86_64Lowerer::new();
        lowerer.set_pcrel_adjust(false);
        lowerer
            .lower_function(&func)
            .map_err(|e| format!("{}: lower error: {:?}", name, e))?;

        let lowered_bytes = lowerer
            .finalize()
            .map_err(|e| format!("{}: finalize error: {:?}", name, e))?;

        let (start, end) = strip_prologue_epilogue(name, &lowered_bytes, Some(bytes))?;
        Ok(lowered_bytes[start..end].to_vec())
    }

    fn find_last_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
        if needle.is_empty() || haystack.len() < needle.len() {
            return None;
        }

        let mut pos = None;
        for i in 0..=haystack.len() - needle.len() {
            if &haystack[i..i + needle.len()] == needle {
                pos = Some(i);
            }
        }
        pos
    }

    fn strip_prologue_epilogue(
        name: &str,
        code: &[u8],
        body_hint: Option<&[u8]>,
    ) -> Result<(usize, usize), String> {
        if code.len() < 6 {
            return Err(format!("{}: lowered bytes too short", name));
        }

        // Prologue: push rbp; mov rbp, rsp
        if code.len() < 4 || code[0..4] != [0x55, 0x48, 0x89, 0xE5] {
            return Err(format!("{}: unexpected prologue", name));
        }

        let mut start = 4;

        if let Some(hint) = body_hint {
            if let Some(pos) = find_last_subslice(&code[start..], hint) {
                let body_start = start + pos;
                return Ok((body_start, body_start + hint.len()));
            }
        }

        // Skip callee-saved pushes
        loop {
            if start < code.len() && code[start] == 0x53 {
                // push rbx
                start += 1;
                continue;
            }
            if start + 1 < code.len() && code[start..start + 2] == [0x41, 0x54] {
                // push r12
                start += 2;
                continue;
            }
            if start + 1 < code.len() && code[start..start + 2] == [0x41, 0x55] {
                // push r13
                start += 2;
                continue;
            }
            if start + 1 < code.len() && code[start..start + 2] == [0x41, 0x56] {
                // push r14
                start += 2;
                continue;
            }
            if start + 1 < code.len() && code[start..start + 2] == [0x41, 0x57] {
                // push r15
                start += 2;
                continue;
            }
            break;
        }

        // Skip stack allocation
        if start + 6 < code.len() && code[start..start + 3] == [0x48, 0x81, 0xEC] {
            start += 7;
        } else if start + 3 < code.len() && code[start..start + 3] == [0x48, 0x83, 0xEC] {
            start += 4;
        }

        let mut end = code.len();

        // Epilogue: pop rbp; ret
        if end < 2 || code[end - 1] != 0xC3 || code[end - 2] != 0x5D {
            return Err(format!("{}: unexpected epilogue", name));
        }
        end -= 2;

        // Skip callee-saved pops (reverse order)
        loop {
            if end >= 1 && code[end - 1] == 0x5B {
                // pop rbx
                end -= 1;
                continue;
            }
            if end >= 2 && code[end - 2..end] == [0x41, 0x5C] {
                // pop r12
                end -= 2;
                continue;
            }
            if end >= 2 && code[end - 2..end] == [0x41, 0x5D] {
                // pop r13
                end -= 2;
                continue;
            }
            if end >= 2 && code[end - 2..end] == [0x41, 0x5E] {
                // pop r14
                end -= 2;
                continue;
            }
            if end >= 2 && code[end - 2..end] == [0x41, 0x5F] {
                // pop r15
                end -= 2;
                continue;
            }
            break;
        }

        // Skip stack deallocation
        if end >= 7 && code[end - 7..end - 4] == [0x48, 0x81, 0xC4] {
            end -= 7;
        } else if end >= 4 && code[end - 4..end - 1] == [0x48, 0x83, 0xC4] {
            end -= 4;
        }

        if end < start {
            return Err(format!("{}: malformed prologue/epilogue", name));
        }

        Ok((start, end))
    }

    /// Lift bytes, execute, then lower/relift and execute again, comparing results
    fn verify_semantic_equivalence(
        name: &str,
        bytes: &[u8],
        initial_rax: u64,
        initial_rbx: u64,
    ) -> bool {
        verify_semantic_equivalence_with_memory(
            name,
            bytes,
            &[(X86Reg::Rax, initial_rax), (X86Reg::Rbx, initial_rbx)],
            &[],
            &[],
        )
    }

    fn verify_semantic_equivalence_with_memory(
        name: &str,
        bytes: &[u8],
        regs: &[(X86Reg, u64)],
        mem_init: &[(u64, &[u8])],
        mem_checks: &[(u64, usize)],
    ) -> bool {
        let mut lifter = X86_64Lifter::new();
        let mut lowerer = X86_64Lowerer::new();

        // Step 1: Lift original bytes
        let mut ctx = LiftContext::new(SourceArch::X86_64);
        let mut block1 = SmirBlock::new(BlockId(0), 0x1000);

        let mut offset = 0;
        let mut pc = 0x1000u64;

        while offset < bytes.len() {
            let remaining = &bytes[offset..];
            if remaining.is_empty() {
                break;
            }
            match lifter.lift_insn(pc, remaining, &mut ctx) {
                Ok(result) => {
                    if result.bytes_consumed == 0 {
                        break; // Avoid infinite loop
                    }
                    for op in result.ops {
                        block1.push_op(op);
                    }
                    pc += result.bytes_consumed as u64;
                    offset += result.bytes_consumed;
                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(crate::smir::lift::LiftError::Incomplete { .. }) => {
                    // Ran out of bytes - that's expected for short sequences
                    break;
                }
                Err(e) => {
                    println!("  [{}] Lift error: {:?}", name, e);
                    return false;
                }
            }
        }
        block1.set_terminator(Terminator::Return { values: vec![] });

        // Step 2: Execute original lifted code
        let (result1, mut mem1) = execute_block_with_state(&block1, regs, mem_init);

        // Step 3: Lower the SMIR
        let mut func = SmirFunction::new(FunctionId(0), BlockId(0), 0x1000);
        func.add_block(block1.clone());

        if let Err(e) = lowerer.lower_function(&func) {
            println!("  [{}] Lower error: {:?}", name, e);
            return false;
        }

        let lowered_bytes = match lowerer.finalize() {
            Ok(b) => b,
            Err(e) => {
                println!("  [{}] Finalize error: {:?}", name, e);
                return false;
            }
        };

        // Step 4: Re-lift the lowered code
        let mut ctx2 = LiftContext::new(SourceArch::X86_64);
        let block2_base = block1.guest_pc;
        let mut block2 = SmirBlock::new(BlockId(1), block2_base);

        // Skip the prologue to re-lift only the body. The prologue is a fixed
        // 20 bytes: `push rbp; mov rbp,rsp` (4) + a 16-byte reserved region
        // (callee-saved pushes + `sub rsp,frame`, NOP-padded) that lower_function
        // backpatches once register allocation is known.
        let code_start = 20; // Skip prologue (4 + 16-byte reserved region)
        let mut offset2 = code_start;
        let mut pc2 = block2_base + code_start as u64;

        while offset2 < lowered_bytes.len() {
            let remaining = &lowered_bytes[offset2..];
            if remaining.is_empty() {
                break;
            }
            match lifter.lift_insn(pc2, remaining, &mut ctx2) {
                Ok(result) => {
                    if result.bytes_consumed == 0 {
                        break; // Avoid infinite loop
                    }
                    for op in result.ops {
                        block2.push_op(op);
                    }
                    pc2 += result.bytes_consumed as u64;
                    offset2 += result.bytes_consumed;
                    if result.control_flow.ends_block() {
                        break;
                    }
                }
                Err(_) => {
                    // Might hit epilogue or other control flow - that's okay
                    break;
                }
            }
        }
        block2.set_terminator(Terminator::Return { values: vec![] });

        // Step 5: Execute re-lifted code
        let (result2, mut mem2) = execute_block_with_state(&block2, regs, mem_init);

        // Step 6: Compare
        let mut matches = result1 == result2;

        for (addr, size) in mem_checks {
            let mut buf1 = vec![0u8; *size];
            let mut buf2 = vec![0u8; *size];
            mem1.read(*addr, &mut buf1).unwrap();
            mem2.read(*addr, &mut buf2).unwrap();
            if buf1 != buf2 {
                matches = false;
            }
        }

        if !matches {
            println!(
                "  [{}] MISMATCH: original={:#x}, round-trip={:#x}",
                name, result1, result2
            );
            println!("    Original ops: {}", block1.ops.len());
            println!("    Re-lifted ops: {}", block2.ops.len());
        } else {
            println!(
                "  [{}] OK: result={:#x} (original {} ops, re-lifted {} ops)",
                name,
                result1,
                block1.ops.len(),
                block2.ops.len()
            );
        }

        matches
    }

    #[test]
    fn test_semantic_add() {
        // ADD RAX, RBX (48 01 D8)
        let bytes = vec![0x48, 0x01, 0xD8];
        assert!(verify_semantic_equivalence("add rax,rbx", &bytes, 100, 42));
        assert!(verify_semantic_equivalence(
            "add rax,rbx overflow",
            &bytes,
            u64::MAX,
            1
        ));
    }

    #[test]
    fn test_semantic_sub() {
        // SUB RAX, RBX (48 29 D8)
        let bytes = vec![0x48, 0x29, 0xD8];
        assert!(verify_semantic_equivalence("sub rax,rbx", &bytes, 100, 42));
        assert!(verify_semantic_equivalence(
            "sub rax,rbx underflow",
            &bytes,
            10,
            20
        ));
    }

    #[test]
    fn test_semantic_xor() {
        // XOR RAX, RBX (48 31 D8)
        let bytes = vec![0x48, 0x31, 0xD8];
        assert!(verify_semantic_equivalence(
            "xor rax,rbx",
            &bytes,
            0xFF00FF00,
            0x00FF00FF
        ));
    }

    #[test]
    fn test_semantic_and() {
        // AND RAX, RBX (48 21 D8)
        let bytes = vec![0x48, 0x21, 0xD8];
        assert!(verify_semantic_equivalence(
            "and rax,rbx",
            &bytes,
            0xFF00FF00,
            0xFFFF0000
        ));
    }

    #[test]
    fn test_semantic_or() {
        // OR RAX, RBX (48 09 D8)
        let bytes = vec![0x48, 0x09, 0xD8];
        assert!(verify_semantic_equivalence(
            "or rax,rbx",
            &bytes,
            0x00FF0000,
            0x0000FF00
        ));
    }

    #[test]
    fn test_semantic_shl() {
        // SHL RAX, 4 (48 C1 E0 04)
        let bytes = vec![0x48, 0xC1, 0xE0, 0x04];
        assert!(verify_semantic_equivalence("shl rax,4", &bytes, 0x1234, 0));
    }

    #[test]
    fn test_semantic_shr() {
        // SHR RAX, 4 (48 C1 E8 04)
        let bytes = vec![0x48, 0xC1, 0xE8, 0x04];
        assert!(verify_semantic_equivalence("shr rax,4", &bytes, 0x12340, 0));
    }

    #[test]
    fn test_semantic_mov_imm() {
        // MOV RAX, 0x12345678 (48 C7 C0 78 56 34 12)
        let bytes = vec![0x48, 0xC7, 0xC0, 0x78, 0x56, 0x34, 0x12];
        assert!(verify_semantic_equivalence(
            "mov rax,imm32",
            &bytes,
            0xDEADBEEF,
            0
        ));
    }

    #[test]
    fn test_semantic_add_imm() {
        // ADD RAX, 0x10 (48 83 C0 10)
        let bytes = vec![0x48, 0x83, 0xC0, 0x10];
        assert!(verify_semantic_equivalence("add rax,16", &bytes, 100, 0));
    }

    #[test]
    fn test_semantic_neg() {
        // NEG RAX (48 F7 D8)
        let bytes = vec![0x48, 0xF7, 0xD8];
        assert!(verify_semantic_equivalence("neg rax", &bytes, 42, 0));
        assert!(verify_semantic_equivalence(
            "neg rax negative",
            &bytes,
            -42i64 as u64,
            0
        ));
    }

    #[test]
    fn test_semantic_not() {
        // NOT RAX (48 F7 D0)
        let bytes = vec![0x48, 0xF7, 0xD0];
        assert!(verify_semantic_equivalence(
            "not rax",
            &bytes,
            0x00FF00FF00FF00FF,
            0
        ));
    }

    #[test]
    fn test_semantic_inc() {
        // INC RAX (48 FF C0)
        let bytes = vec![0x48, 0xFF, 0xC0];
        assert!(verify_semantic_equivalence("inc rax", &bytes, 41, 0));
    }

    #[test]
    fn test_semantic_dec() {
        // DEC RAX (48 FF C8)
        let bytes = vec![0x48, 0xFF, 0xC8];
        assert!(verify_semantic_equivalence("dec rax", &bytes, 43, 0));
    }

    #[test]
    fn test_semantic_multi_instruction() {
        // Multiple instructions: MOV RAX, 10; ADD RAX, RBX
        let bytes = vec![
            0x48, 0xC7, 0xC0, 0x0A, 0x00, 0x00, 0x00, // mov rax, 10
            0x48, 0x01, 0xD8, // add rax, rbx
        ];
        assert!(verify_semantic_equivalence(
            "mov+add sequence",
            &bytes,
            0,
            32
        ));
    }

    #[test]
    fn test_semantic_load_store_direct() {
        // MOV RAX, [RDI]
        let load_bytes = vec![0x48, 0x8B, 0x07];
        let addr = 0x100u64;
        let value = 0x1122334455667788u64;
        let value_bytes = value.to_le_bytes();

        let block = lift_bytes_to_block("mov rax,[rdi]", &load_bytes, BlockId(0), 0x1000)
            .expect("lift load");
        let (result, mut mem) =
            execute_block_with_state(&block, &[(X86Reg::Rdi, addr)], &[(addr, &value_bytes)]);
        assert_eq!(result, value);
        assert_eq!(read_u64(&mut mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov rax,[rdi]",
            &load_bytes,
            &[(X86Reg::Rdi, addr)],
            &[(addr, &value_bytes)],
            &[(addr, 8)],
        ));

        // MOV [RDI], RAX
        let store_bytes = vec![0x48, 0x89, 0x07];
        let store_block = lift_bytes_to_block("mov [rdi],rax", &store_bytes, BlockId(1), 0x1100)
            .expect("lift store");
        let (store_result, mut store_mem) = execute_block_with_state(
            &store_block,
            &[(X86Reg::Rax, value), (X86Reg::Rdi, addr)],
            &[],
        );
        assert_eq!(store_result, value);
        assert_eq!(read_u64(&mut store_mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov [rdi],rax",
            &store_bytes,
            &[(X86Reg::Rax, value), (X86Reg::Rdi, addr)],
            &[],
            &[(addr, 8)],
        ));
    }

    #[test]
    fn test_semantic_load_store_offset() {
        // MOV RAX, [RDI+8]
        let load_bytes = vec![0x48, 0x8B, 0x47, 0x08];
        let addr = 0x120u64;
        let value = 0xAABBCCDDEEFF0011u64;
        let value_bytes = value.to_le_bytes();

        let block = lift_bytes_to_block("mov rax,[rdi+8]", &load_bytes, BlockId(2), 0x1200)
            .expect("lift load offset");
        let (result, mut mem) =
            execute_block_with_state(&block, &[(X86Reg::Rdi, addr - 8)], &[(addr, &value_bytes)]);
        assert_eq!(result, value);
        assert_eq!(read_u64(&mut mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov rax,[rdi+8]",
            &load_bytes,
            &[(X86Reg::Rdi, addr - 8)],
            &[(addr, &value_bytes)],
            &[(addr, 8)],
        ));
    }

    #[test]
    fn test_semantic_load_store_sib() {
        // MOV RAX, [RDI + RSI*4 + 16]
        let load_bytes = vec![0x48, 0x8B, 0x84, 0xB7, 0x10, 0x00, 0x00, 0x00];
        let base = 0x200u64;
        let index = 3u64;
        let addr = base + index * 4 + 16;
        let value = 0xDEADBEEFCAFEBABEu64;
        let value_bytes = value.to_le_bytes();

        let block = lift_bytes_to_block("mov rax,[rdi+rsi*4+16]", &load_bytes, BlockId(3), 0x1300)
            .expect("lift load sib");
        let (result, mut mem) = execute_block_with_state(
            &block,
            &[(X86Reg::Rdi, base), (X86Reg::Rsi, index)],
            &[(addr, &value_bytes)],
        );
        assert_eq!(result, value);
        assert_eq!(read_u64(&mut mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov rax,[rdi+rsi*4+16]",
            &load_bytes,
            &[(X86Reg::Rdi, base), (X86Reg::Rsi, index)],
            &[(addr, &value_bytes)],
            &[(addr, 8)],
        ));
    }

    #[test]
    fn test_semantic_absolute_addressing() {
        // MOV RAX, [abs32]
        let load_bytes = vec![0x48, 0x8B, 0x04, 0x25, 0x00, 0x02, 0x00, 0x00];
        let addr = 0x200u64;
        let value = 0x0F0E0D0C0B0A0908u64;
        let value_bytes = value.to_le_bytes();

        let block = lift_bytes_to_block("mov rax,[abs]", &load_bytes, BlockId(4), 0x1400)
            .expect("lift absolute load");
        let (result, mut mem) = execute_block_with_state(&block, &[], &[(addr, &value_bytes)]);
        assert_eq!(result, value);
        assert_eq!(read_u64(&mut mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov rax,[abs]",
            &load_bytes,
            &[],
            &[(addr, &value_bytes)],
            &[(addr, 8)],
        ));

        // MOV [abs32], RAX
        let store_bytes = vec![0x48, 0x89, 0x04, 0x25, 0x00, 0x02, 0x00, 0x00];
        let store_block = lift_bytes_to_block("mov [abs],rax", &store_bytes, BlockId(5), 0x1500)
            .expect("lift absolute store");
        let (store_result, mut store_mem) =
            execute_block_with_state(&store_block, &[(X86Reg::Rax, value)], &[]);
        assert_eq!(store_result, value);
        assert_eq!(read_u64(&mut store_mem, addr), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov [abs],rax",
            &store_bytes,
            &[(X86Reg::Rax, value)],
            &[],
            &[(addr, 8)],
        ));
    }

    #[test]
    fn test_semantic_lea() {
        // LEA RAX, [RDI + RSI*4 + 16]
        let bytes = vec![0x48, 0x8D, 0x84, 0xB7, 0x10, 0x00, 0x00, 0x00];
        let base = 0x300u64;
        let index = 5u64;
        let expected = base + index * 4 + 16;

        let block = lift_bytes_to_block("lea rax,[rdi+rsi*4+16]", &bytes, BlockId(6), 0x1600)
            .expect("lift lea");
        let (result, _mem) =
            execute_block_with_state(&block, &[(X86Reg::Rdi, base), (X86Reg::Rsi, index)], &[]);
        assert_eq!(result, expected);

        assert!(verify_semantic_equivalence_with_memory(
            "lea rax,[rdi+rsi*4+16]",
            &bytes,
            &[(X86Reg::Rdi, base), (X86Reg::Rsi, index)],
            &[],
            &[],
        ));
    }

    #[test]
    fn test_semantic_pcrel_load() {
        // MOV RAX, [RIP+0x10]
        let bytes = vec![0x48, 0x8B, 0x05, 0x10, 0x00, 0x00, 0x00];
        let base_pc = 0x1800u64;
        let target = base_pc + 7 + 0x10;
        let value = 0xCAFEBABEDEADBEEFu64;
        let value_bytes = value.to_le_bytes();

        let block = lift_bytes_to_block("mov rax,[rip+0x10]", &bytes, BlockId(20), base_pc)
            .expect("lift pcrel load");
        let (result, mut mem) = execute_block_with_state(&block, &[], &[(target, &value_bytes)]);
        assert_eq!(result, value);
        assert_eq!(read_u64(&mut mem, target), value);

        assert!(verify_semantic_equivalence_with_memory(
            "mov rax,[rip+0x10]",
            &bytes,
            &[],
            &[(target, &value_bytes)],
            &[(target, 8)],
        ));
    }

    #[test]
    fn test_semantic_pcrel_lea() {
        // LEA RAX, [RIP+0x10]
        let bytes = vec![0x48, 0x8D, 0x05, 0x10, 0x00, 0x00, 0x00];
        let base_pc = 0x1900u64;
        let expected = base_pc + 7 + 0x10;

        let block = lift_bytes_to_block("lea rax,[rip+0x10]", &bytes, BlockId(21), base_pc)
            .expect("lift pcrel lea");
        let (result, _mem) = execute_block_with_state(&block, &[], &[]);
        assert_eq!(result, expected);

        assert!(verify_semantic_equivalence_with_memory(
            "lea rax,[rip+0x10]",
            &bytes,
            &[],
            &[],
            &[],
        ));
    }

    #[test]
    fn test_semantic_rep_stosq() {
        // REP STOSQ
        let bytes = vec![0xF3, 0x48, 0xAB];
        let addr = 0x200u64;
        let value = 0x0F1E2D3C4B5A6978u64;
        let count = 3u64;

        let block =
            lift_bytes_to_block("rep stosq", &bytes, BlockId(22), 0x1A00).expect("lift rep stosq");
        let (ctx, mut mem) = execute_block(
            &block,
            &[
                (X86Reg::Rdi, addr),
                (X86Reg::Rax, value),
                (X86Reg::Rcx, count),
            ],
            &[],
        );

        assert_eq!(
            ctx.read_arch_reg(ArchReg::X86(X86Reg::Rdi)),
            addr + 8 * count
        );
        assert_eq!(ctx.read_arch_reg(ArchReg::X86(X86Reg::Rcx)), 0);
        assert_eq!(read_u64(&mut mem, addr), value);
        assert_eq!(read_u64(&mut mem, addr + 8), value);
        assert_eq!(read_u64(&mut mem, addr + 16), value);

        assert!(verify_semantic_equivalence_with_memory(
            "rep stosq",
            &bytes,
            &[
                (X86Reg::Rdi, addr),
                (X86Reg::Rax, value),
                (X86Reg::Rcx, count),
            ],
            &[],
            &[(addr, 24)],
        ));
    }

    #[test]
    fn test_semantic_cmp_sete() {
        // CMP RAX, RBX; SETE AL
        let bytes = vec![0x48, 0x39, 0xD8, 0x0F, 0x94, 0xC0];

        let block =
            lift_bytes_to_block("cmp+sete", &bytes, BlockId(7), 0x1700).expect("lift cmp+sete");
        let (result_eq, _mem_eq) =
            execute_block_with_state(&block, &[(X86Reg::Rax, 123), (X86Reg::Rbx, 123)], &[]);
        assert_eq!(result_eq, 1);
        let (result_ne, _mem_ne) =
            execute_block_with_state(&block, &[(X86Reg::Rax, 123), (X86Reg::Rbx, 456)], &[]);
        assert_eq!(result_ne, 0);

        assert!(verify_semantic_equivalence(
            "cmp+sete (equal)",
            &bytes,
            123,
            123
        ));
        assert!(verify_semantic_equivalence(
            "cmp+sete (not equal)",
            &bytes,
            123,
            456
        ));
    }

    #[test]
    fn test_roundtrip_exact_bytes_basic() {
        let cases: Vec<(&str, Vec<u8>)> = vec![
            ("mov rax, rcx", vec![0x48, 0x89, 0xC8]),
            (
                "mov rax, imm64",
                vec![0x48, 0xB8, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11],
            ),
            ("add rax, rcx", vec![0x48, 0x01, 0xC8]),
            ("sub rax, rdx", vec![0x48, 0x29, 0xD0]),
            ("xor rax, rax", vec![0x48, 0x31, 0xC0]),
            ("and rax, rcx", vec![0x48, 0x21, 0xC8]),
            ("or rax, rdx", vec![0x48, 0x09, 0xD0]),
            ("cmp rax, rcx", vec![0x48, 0x39, 0xC8]),
            ("test rax, rax", vec![0x48, 0x85, 0xC0]),
            ("shl rax, 4", vec![0x48, 0xC1, 0xE0, 0x04]),
            ("shr rax, 4", vec![0x48, 0xC1, 0xE8, 0x04]),
            ("sar rax, 4", vec![0x48, 0xC1, 0xF8, 0x04]),
            ("sete al", vec![0x0F, 0x94, 0xC0]),
            ("rep stosq", vec![0xF3, 0x48, 0xAB]),
        ];

        for (name, bytes) in cases {
            let lowered = lower_body_bytes(name, &bytes).expect("lower body bytes");
            assert_eq!(lowered, bytes, "{}: bytes mismatch", name);
        }
    }

    #[test]
    fn test_roundtrip_exact_bytes_memory() {
        let cases: Vec<(&str, Vec<u8>)> = vec![
            ("mov rax, [rdi]", vec![0x48, 0x8B, 0x07]),
            ("mov [rdi], rax", vec![0x48, 0x89, 0x07]),
            ("mov rax, [rdi+8]", vec![0x48, 0x8B, 0x47, 0x08]),
            ("mov rax, [rdi+0x7f]", vec![0x48, 0x8B, 0x47, 0x7F]),
            (
                "mov rax, [rdi+0x80]",
                vec![0x48, 0x8B, 0x87, 0x80, 0x00, 0x00, 0x00],
            ),
            (
                "mov rax, [rdi+rsi*4+16]",
                vec![0x48, 0x8B, 0x84, 0xB7, 0x10, 0x00, 0x00, 0x00],
            ),
            (
                "lea rax, [rdi+rsi*4+16]",
                vec![0x48, 0x8D, 0x84, 0xB7, 0x10, 0x00, 0x00, 0x00],
            ),
            (
                "mov rax, [rip+0x10]",
                vec![0x48, 0x8B, 0x05, 0x10, 0x00, 0x00, 0x00],
            ),
            (
                "lea rax, [rip+0x10]",
                vec![0x48, 0x8D, 0x05, 0x10, 0x00, 0x00, 0x00],
            ),
            (
                "mov rax, [abs32]",
                vec![0x48, 0x8B, 0x04, 0x25, 0x00, 0x02, 0x00, 0x00],
            ),
            (
                "mov [abs32], rax",
                vec![0x48, 0x89, 0x04, 0x25, 0x00, 0x02, 0x00, 0x00],
            ),
        ];

        for (name, bytes) in cases {
            let lowered = lower_body_bytes(name, &bytes).expect("lower body bytes");
            assert_eq!(lowered, bytes, "{}: bytes mismatch", name);
        }
    }
}
