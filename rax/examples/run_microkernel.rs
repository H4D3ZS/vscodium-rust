//! Microkernel runner - loads and runs the bare-metal microkernel example.
//!
//! This demonstrates running a complete bare-metal Rust binary in the emulator.

use std::sync::Arc;

use vm_memory::{Bytes, GuestAddress, GuestMemoryMmap, GuestRegionMmap, MmapRegion};

use rax::backend::emulator::x86_64::X86_64Vcpu;
use rax::cpu::{Registers, SystemRegisters, VCpu, VcpuExit};

const LOAD_ADDR: u64 = 0x100000;
const MEM_SIZE: u64 = 16 * 1024 * 1024; // 16 MB

fn main() {
    println!("==============================================");
    println!("  RAX Emulator - Microkernel Example Runner");
    println!("==============================================\n");

    // Load the microkernel binary
    let bin_path = concat!(env!("CARGO_MANIFEST_DIR"), "/microkernel/microkernel.bin");

    let code = match std::fs::read(bin_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read microkernel binary: {}", e);
            eprintln!("Make sure to build it first:");
            eprintln!("  cd microkernel && cargo +nightly build --release");
            eprintln!(
                "  llvm-objcopy -O binary target/x86_64-unknown-none/release/microkernel microkernel.bin"
            );
            std::process::exit(1);
        }
    };

    println!("[LOADER] Binary: {} ({} bytes)", bin_path, code.len());
    println!("[LOADER] Load address: {:#x}", LOAD_ADDR);

    // Create guest memory
    let region = MmapRegion::new(MEM_SIZE as usize).expect("Failed to create mmap region");
    let guest_region =
        GuestRegionMmap::new(region, GuestAddress(0)).expect("Failed to create guest region");
    let memory = Arc::new(
        GuestMemoryMmap::from_regions(vec![guest_region]).expect("Failed to create guest memory"),
    );

    println!("[LOADER] Guest memory: {} MB", MEM_SIZE / (1024 * 1024));

    // Load binary at LOAD_ADDR
    memory
        .write_slice(&code, GuestAddress(LOAD_ADDR))
        .expect("Failed to write code to memory");

    // Set up initial CPU state for 64-bit long mode
    let mut regs = Registers::default();
    regs.rip = LOAD_ADDR;
    regs.rsp = 0x110000; // Stack at 0x110000 (per linker script)
    regs.rflags = 0x2; // Reserved bit must be 1

    let mut sregs = SystemRegisters::default();
    // Set up 64-bit long mode
    sregs.cr0 = 0x21; // PE | NE (no paging)
    sregs.cr4 = 0x20; // PAE
    sregs.efer = 0x500; // LMA | LME (long mode active)

    // Code segment for 64-bit mode
    sregs.cs.base = 0;
    sregs.cs.limit = 0xFFFFFFFF;
    sregs.cs.selector = 0x8;
    sregs.cs.type_ = 0xB; // Execute/Read, accessed
    sregs.cs.present = true;
    sregs.cs.dpl = 0;
    sregs.cs.db = false;
    sregs.cs.s = true;
    sregs.cs.l = true; // 64-bit code segment
    sregs.cs.g = true;

    // Data segments
    sregs.ds.base = 0;
    sregs.ds.limit = 0xFFFFFFFF;
    sregs.ds.selector = 0x10;
    sregs.ds.type_ = 0x3; // Read/Write, accessed
    sregs.ds.present = true;
    sregs.ds.dpl = 0;
    sregs.ds.db = true;
    sregs.ds.s = true;
    sregs.ds.g = true;

    sregs.es = sregs.ds.clone();
    sregs.fs = sregs.ds.clone();
    sregs.gs = sregs.ds.clone();
    sregs.ss = sregs.ds.clone();

    // Create vCPU and set initial state
    let mut vcpu = X86_64Vcpu::new(0, memory);
    vcpu.set_regs(&regs).expect("Failed to set registers");
    vcpu.set_sregs(&sregs)
        .expect("Failed to set system registers");

    println!("[CPU] Initial RIP: {:#x}", regs.rip);
    println!("[CPU] Initial RSP: {:#x}", regs.rsp);
    println!("\n[EMU] Starting execution...\n");
    println!("---------- MICROKERNEL OUTPUT ----------\n");

    // Run the microkernel using step() for precise control
    let mut insn_count: u64 = 0;
    let max_insns: u64 = 100_000_000; // 100 million instructions max
    let trace_first_n = 0; // Set to non-zero to trace first N instructions

    loop {
        if insn_count < trace_first_n {
            let r = vcpu.get_regs().unwrap();
            eprintln!(
                "[{:6}] RIP={:#x} RSP={:#x} RAX={:#x}",
                insn_count, r.rip, r.rsp, r.rax
            );
        }
        match vcpu.step() {
            Ok(Some(VcpuExit::Hlt)) => {
                println!("\n--------- END MICROKERNEL OUTPUT --------\n");
                println!("[EMU] Halted after {} instructions", insn_count);
                break;
            }
            Ok(Some(VcpuExit::IoOut { port, data })) => {
                if port == 0xE9 {
                    // Debug output port
                    for &b in &data {
                        print!("{}", b as char);
                    }
                    // Flush stdout for immediate output
                    use std::io::Write;
                    let _ = std::io::stdout().flush();
                }
            }
            Ok(Some(VcpuExit::IoIn { size, .. })) => {
                // Complete I/O with zeros
                let data = vec![0u8; size as usize];
                vcpu.complete_io_in(&data);
            }
            Ok(Some(exit)) => {
                println!("\n[EMU] Unhandled exit: {:?}", exit);
                break;
            }
            Ok(None) => {
                // Instruction executed normally, continue
            }
            Err(e) => {
                println!("\n[EMU] Error: {}", e);
                println!("[EMU] Instruction count: {}", insn_count);

                // Debug state
                if let Ok(regs) = vcpu.get_regs() {
                    println!("\n[DEBUG] CPU State:");
                    println!("  RIP: {:#x}", regs.rip);
                    println!("  RAX: {:#x}", regs.rax);
                    println!("  RBX: {:#x}", regs.rbx);
                    println!("  RCX: {:#x}", regs.rcx);
                    println!("  RDX: {:#x}", regs.rdx);
                    println!("  RSI: {:#x}", regs.rsi);
                    println!("  RDI: {:#x}", regs.rdi);
                    println!("  RSP: {:#x}", regs.rsp);
                    println!("  RBP: {:#x}", regs.rbp);
                }
                std::process::exit(1);
            }
        }
        insn_count += 1;
        if insn_count >= max_insns {
            println!("\n[EMU] Reached max instruction count ({})", max_insns);
            break;
        }
    }

    println!("\n[EMU] Microkernel execution complete!");
    println!("[EMU] Total instructions executed: {}", insn_count);
}
